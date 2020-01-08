extern crate bincode;
extern crate ethereum_types;
extern crate serde;
extern crate serde_derive;
extern crate sha3;

use durian::state_provider::{Error, StateAccount, StateProvider};
use ethereum_types::{Address, H256, U256, U512};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::time::SystemTime;

pub type Hash = H256;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Block {
    num: u32,
    prev: Hash,
    time: SystemTime,
}

#[derive(Debug)]
pub struct Account {
    address: Address,
    nonce: U256,
    balance: U256,
    code: Vec<u8>,
    storage: HashMap<H256, H256>,
}

#[derive(Debug)]
pub struct Blockchain<'a> {
    blocks: Vec<Block>,
    accounts: HashMap<&'a str, Account>,
    counter: i32,
}

impl<'a> Blockchain<'a> {
    pub fn new() -> Blockchain<'a> {
        let gen = Block::new(0, Hash::zero());
        let mut accounts = HashMap::new();
        accounts.insert(
            "alice",
            Account::new(Address::random(), U256::from(1000000)),
        );
        accounts.insert("bob", Account::new(Address::random(), U256::from(1000000)));
        accounts.insert(
            "carol",
            Account::new(Address::random(), U256::from(1000000)),
        );
        accounts.insert("dave", Account::new(Address::random(), U256::from(1000000)));

        Blockchain {
            blocks: vec![gen],
            accounts: accounts,
            counter: 0,
        }
    }

    pub fn commit(&mut self) {
        let block = Block::new(self.blocks.len() as u32, self.blocks.last().unwrap().hash());

        self.blocks.push(block);
    }

    pub fn address(&self, alias: &str) -> Address {
        self.accounts.get(alias).unwrap().address
    }

    pub fn code(&self, alias: &str) -> Vec<u8> {
        self.accounts.get(alias).unwrap().code.clone()
    }

    fn account(&self, address: &Address) -> Result<&Account, Error> {
        for (_, acc) in self.accounts.iter() {
            if acc.address == *address {
                return Ok(acc);
            }
        }

        Err(Error::InvalidAddress)
    }
}

impl<'a> StateProvider for Blockchain<'a> {
    fn account(&self, address: &Address) -> Result<StateAccount, Error> {
        let acc = self.account(address)?;
        Ok(StateAccount {
            balance: U256::from(acc.balance),
            nonce: U256::from(acc.nonce),
            code: acc.code.clone(),
        })
    }

    fn create_account(&mut self, address: Address, account: StateAccount) {
        //let name = format!("contract_{}", self.counter);
        let mut acc = Account::new(address, account.balance);
        acc.code = account.code;
        //self.accounts.insert(&name, acc);
        self.accounts.insert("contract", acc);
        self.counter = self.counter + 1;
    }

    fn storage_at(&self, address: &Address, key: &H256) -> Result<H256, Error> {
        let acc = self.account(address)?;
        match acc.storage.get(key) {
            Some(storage) => Ok(*storage),
            _ => Err(Error::InvalidStorageKey)
        }
    }
    fn set_storage(&mut self, address: &Address, key: &H256, value: &H256) {
        for (_, acc) in self.accounts.iter_mut() {
            if acc.address == *address {
                let val = acc.storage.entry(*key).or_insert(*value);
                *val = *value;            }
        }

       // let acc = self.account(address).unwrap();
        //let val = acc.storage.entry(*key).or_insert(*value);
        //*val = *value;
    }
    fn blockhash(&self, num: i64) -> U512 {
        U512::zero()
    }
    fn exist(&self, address: &Address) -> bool {
        false
    }
}

impl Block {
    pub fn new(num: u32, prev: Hash) -> Block {
        Block {
            num: num,
            prev: prev,
            time: SystemTime::now(),
        }
    }

    pub fn hash(&self) -> Hash {
        let bytes = bincode::serialize(self).unwrap();
        Hash::from_slice(Keccak256::digest(&bytes).as_slice())
    }
}

impl Account {
    pub fn new(addr: Address, bal: U256) -> Account {
        Account {
            address: addr,
            balance: bal,
            nonce: U256::from(0),
            code: vec![],
            storage: HashMap::new(),
        }
    }
}
