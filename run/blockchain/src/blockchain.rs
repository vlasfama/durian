use crate::account::Account;
use crate::block::Block;
use durian::error::{Error, Result};
use durian::execute::ResultData;
use durian::provider::{Provider, StateAccount};
use durian::transaction::{Action, Transaction};
use ethereum_types::{Address, H160, H256, U256};
use hex_literal::hex;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>,
    accounts: HashMap<String, Account>,
    counter: i32,
    transactions: HashMap<H256, (Transaction, ResultData)>,
}

pub fn transaction_hash(transaction: &Transaction) -> H256 {
    let mut bytes = Vec::new();
    let mut tmp = Vec::new();
    tmp.resize(32, 0);

    bytes.extend_from_slice(transaction.sender.as_bytes());

    transaction.value.to_little_endian(&mut tmp);
    bytes.extend_from_slice(tmp.as_slice());

    transaction.gas.to_little_endian(&mut tmp);
    bytes.extend_from_slice(tmp.as_slice());

    transaction.gas_price.to_little_endian(&mut tmp);
    bytes.extend_from_slice(tmp.as_slice());

    match &transaction.action {
        Action::Create(code, salt) => {
            bytes.extend_from_slice(code.as_slice());
            bytes.extend_from_slice(salt.as_bytes());
        }
        Action::Call(address) => {
            bytes.extend_from_slice(address.as_bytes());
        }
    }

    bytes.extend_from_slice(transaction.args.as_slice());

    H256::from_slice(Keccak256::digest(&bytes).as_slice())
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let gen = Block::new(0, H256::zero());
        let mut accounts = HashMap::new();
        let addr = Address::from_slice(&hex!("004ec07d2329997267ec62b4166639513386f32e")[..]);

        accounts.insert(
            "alice".to_string(),
            Account::new(Address::random(), U256::from(1000000), U256::zero(), vec![]),
        );
        accounts.insert(
            "bob".to_string(),
            Account::new(Address::random(), U256::from(1000000), U256::zero(), vec![]),
        );
        accounts.insert(
            "carol".to_string(),
            Account::new(Address::random(), U256::from(1000000), U256::zero(), vec![]),
        );
        accounts.insert(
            "dave".to_string(),
            Account::new(Address::random(), U256::from(1000000), U256::zero(), vec![]),
        );
        accounts.insert(
            "naga".to_string(),
            Account::new(addr, U256::from(1000000), U256::zero(), vec![]),
        );

        Blockchain {
            blocks: vec![gen],
            accounts: accounts,
            counter: 0,
            transactions: HashMap::new(),
        }
    }

    pub fn commit(&mut self) {
        let block = Block::new(self.blocks.len() as u64, self.blocks.last().unwrap().hash());

        info!(
            "Committing new block. num: {}, hash: {}",
            block.num,
            block.hash()
        );
        self.blocks.push(block);

        info!("Accounts:");
        for (alias, acc) in self.accounts.iter() {
            info!(
                "  {:?}: address: {}, balance: {}, nonce:{}",
                alias, acc.address, acc.balance, acc.nonce
            );

            if !acc.storage.is_empty() {
                info!("  Storage:");
                for (key, val) in acc.storage.iter() {
                    info!("    {}...: {}", key, val);
                }
            }
        }
    }

    pub fn address_from_alias(&self, alias: &str) -> Address {
        self.accounts.get(alias).unwrap().address
    }

    pub fn balance(&self, address: H160) -> U256 {
        let acc = self.account(&address).unwrap();
        acc.balance.clone()
    }

    pub fn code(&self, alias: &str) -> Vec<u8> {
        self.accounts.get(alias).unwrap().code.clone()
    }

    pub fn code_at(&self, address: H160) -> Vec<u8> {
        let acc = self.account(&address).unwrap();
        acc.code.clone()
    }

    pub fn inc_nonce(&mut self, alias: &str) {
        let mut acc = self.accounts.get_mut(alias).unwrap();
        acc.nonce = acc.nonce + U256::from(1)
    }

    fn account(&self, address: &Address) -> Result<&Account> {
        for (_, acc) in self.accounts.iter() {
            if acc.address == *address {
                return Ok(acc);
            }
        }

        Err(Error::InvalidAddress { address: *address })
    }

    fn account_mut(&mut self, address: &Address) -> Result<&mut Account> {
        for (_, acc) in self.accounts.iter_mut() {
            if acc.address == *address {
                return Ok(acc);
            }
        }

        Err(Error::InvalidAddress { address: *address })
    }

    pub fn add_transactions(&mut self, transaction: Transaction, result: ResultData) -> H256 {
        let txhash = transaction_hash(&transaction);
        self.transactions.insert(txhash, (transaction, result));
        return txhash;
    }

    pub fn get_transaction_details(&mut self, hash: H256) -> Result<(Transaction, ResultData)> {
        let tx = self.transactions.get(&hash).unwrap();
        return Ok(tx.clone());
    }

    pub fn block_number(&self) -> u64 {
        self.blocks.last().unwrap().num
    }

    pub fn latest_block_hash(&self) -> H256 {
        self.blocks.last().unwrap().hash()
    }
}

impl Provider for Blockchain {
    fn account(&self, address: &Address) -> Result<StateAccount> {
        let acc = self.account(address)?;
        Ok(StateAccount {
            balance: U256::from(acc.balance),
            nonce: U256::from(acc.nonce),
            code: acc.code.clone(),
        })
    }

    fn create_contract(&mut self, address: &Address, code: &Vec<u8>) -> Result<()> {
        let name = format!("contract_{}", self.counter + 1);
        let acc = Account::new(*address, U256::zero(), U256::zero(), code.clone());
        self.accounts.insert(name, acc);
        self.counter = self.counter + 1;
        Ok(())
    }

    fn storage_at(&self, address: &Address, key: &H256) -> Result<H256> {
        let acc = self.account(address)?;
        match acc.storage.get(key) {
            Some(storage) => Ok(*storage),
            _ => Err(Error::InvalidStorageKey { key: *key }),
        }
    }

    fn set_storage(&mut self, address: &Address, key: &H256, value: &H256) -> Result<()> {
        let acc = self.account_mut(address).unwrap();
        let val = acc.storage.entry(*key).or_insert(*value);
        *val = *value;
        Ok(())
    }

    fn block_hash(&self, num: u64) -> Result<H256> {
        Ok(self.blocks.get(num as usize).unwrap().hash())
    }

    fn timestamp(&self) -> u64 {
        let duration_since_epoch = self
            .blocks
            .last()
            .unwrap()
            .time
            .duration_since(SystemTime::UNIX_EPOCH);
        duration_since_epoch.unwrap().as_secs()
    }

    fn block_number(&self) -> u64 {
        self.blocks.last().unwrap().num
    }

    fn block_author(&self) -> Result<Address> {
        Ok(self.address_from_alias("alice"))
    }

    fn difficulty(&self) -> Result<U256> {
        Err(Error::NotSupported)
    }

    fn gas_limit(&self) -> Result<U256> {
        Ok(U256::from(1000000))
    }

    fn exist(&self, address: &Address) -> bool {
        self.account(address).is_ok()
    }

    fn update_account(&mut self, address: &Address, bal: &U256, nonce: &U256) -> Result<()> {
        let mut acc = self.account_mut(address).unwrap();
        acc.balance = *bal;
        acc.nonce = *nonce;
        Ok(())
    }
}
