extern crate bincode;
extern crate durian;
extern crate ethereum_types;
extern crate hex_literal;
extern crate sha3;
extern crate time;

use durian::error::Error;
use durian::state_provider::{StateAccount, StateProvider};
use durian::stateless_vm::ResultData;
use durian::transaction::Transaction;
use ethereum_types::{Address, H160, H256, U256, U512};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use time::PrimitiveDateTime;

use hex_literal::hex;

pub type Hash = H256;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Block {
    num: u32,
    prev: Hash,
    time: PrimitiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    address: Address,
    nonce: U256,
    balance: U256,
    code: Vec<u8>,
    storage: HashMap<H256, H256>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Blockchain {
    blocks: Vec<Block>,
    accounts: HashMap<String, Account>,
    counter: i32,
    pub transactions: HashMap<Hash, (Transaction, ResultData)>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let gen = Block::new(0, Hash::zero());
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
        let block = Block::new(self.blocks.len() as u32, self.blocks.last().unwrap().hash());

        info!(
            "Committing new block. num: {:?}, hash: {:?}",
            block.num,
            block.hash()
        );
        self.blocks.push(block);

        info!("Accounts:");
        for (alias, acc) in self.accounts.iter() {
            info!(
                "  {:?}: address: {:?}, balance: {:?}, nonce:{:?}",
                alias, acc.address, acc.balance, acc.nonce
            );

            if !acc.storage.is_empty() {
                info!("  Storage:");
                for (key, val) in acc.storage.iter() {
                    info!("    {:?}: {:?}", key, val);
                }
            }
        }
    }

    pub fn address(&self, alias: &str) -> Address {
        self.accounts.get(alias).unwrap().address
    }

    pub fn get_balance(&self, address: H160) -> U256 {
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

    pub fn incNonce(&mut self, alias: &str) {
        let mut acc = self.accounts.get_mut(alias).unwrap();
        acc.nonce = acc.nonce + U256::from(1)
    }

    fn account(&self, address: &Address) -> Result<&Account, Error> {
        for (_, acc) in self.accounts.iter() {
            if acc.address == *address {
                return Ok(acc);
            }
        }

        Err(Error::InvalidAddress)
    }

    fn account_mut(&mut self, address: &Address) -> Result<&mut Account, Error> {
        for (_, acc) in self.accounts.iter_mut() {
            if acc.address == *address {
                return Ok(acc);
            }
        }

        Err(Error::InvalidAddress)
    }

    pub fn last_block_hash(&self) -> Hash {
        self.blocks.last().unwrap().hash()
    }

    pub fn get_blocknumber(&self) -> u32 {
        self.blocks.last().unwrap().block_number()
    }

    pub fn add_transactions(&mut self, transaction: Transaction, result: ResultData) -> H256 {
        let txhash = transaction.hash();
        self.transactions.insert(txhash, (transaction, result));
        return txhash;
    }

    pub fn storage_at(&mut self, address: Address, data: H256) -> Result<H256, Error> {
        let sc_deatils = self.storage_at(address, data).unwrap();
        return Ok(sc_deatils.clone());
    }

    pub fn get_transactiondetails(
        &mut self,
        hash: H256,
    ) -> Result<(Transaction, ResultData), Error> {
        let tx = self.transactions.get(&hash).unwrap();
        return Ok(tx.clone());
    }
}

impl StateProvider for Blockchain {
    fn account(&self, address: &Address) -> Result<StateAccount, Error> {
        let acc = self.account(address)?;
        Ok(StateAccount {
            balance: U256::from(acc.balance),
            nonce: U256::from(acc.nonce),
            code: acc.code.clone(),
        })
    }

    fn create_contract(
        &mut self,
        address: &Address,
        nonce: &U256,
        code: &Vec<u8>,
    ) -> Result<(), Error> {
        let name = format!("contract_{}", self.counter + 1);
        let mut acc = Account::new(*address, U256::zero(), *nonce, code.clone());
        self.accounts.insert(name, acc);
        self.counter = self.counter + 1;
        Ok(())
    }

    fn storage_at(&self, address: &Address, key: &H256) -> Result<H256, Error> {
        let acc = self.account(address)?;
        match acc.storage.get(key) {
            Some(storage) => Ok(*storage),
            _ => Err(Error::InvalidStorageKey),
        }
    }

    fn set_storage(&mut self, address: &Address, key: &H256, value: &H256) -> Result<(), Error> {
        let mut acc = self.account_mut(address).unwrap();
        let val = acc.storage.entry(*key).or_insert(*value);
        *val = *value;
        Ok(())
    }

    fn blockhash(&self, num: i64) -> U512 {
        U512::zero()
    }

    fn exist(&self, address: &Address) -> bool {
        false
    }

    fn update_account(&mut self, address: &Address, bal: &U256, nonce: &U256) -> Result<(), Error> {
        let mut acc = self.account_mut(address).unwrap();
        acc.balance = *bal;
        acc.nonce = *nonce;
        Ok(())
    }
}

impl Block {
    pub fn new(num: u32, prev: Hash) -> Block {
        Block {
            num: num,
            prev: prev,
            time: PrimitiveDateTime::now(),
        }
    }

    pub fn hash(&self) -> Hash {
        let bytes = bincode::serialize(self).unwrap();
        Hash::from_slice(Keccak256::digest(&bytes).as_slice())
    }

    pub fn block_number(&self) -> u32 {
        let block_number = self.num;
        return block_number;
    }
}

impl Account {
    pub fn new(addr: Address, bal: U256, nonce: U256, code: Vec<u8>) -> Account {
        Account {
            address: addr,
            balance: bal,
            nonce: nonce,
            code: code,
            storage: HashMap::new(),
        }
    }
}
