extern crate bincode;
extern crate ethereum_types;
extern crate serde;
extern crate serde_derive;
extern crate sha3;

use durian::state_provider::StateProvider;
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
    balance: u64,
    code: Option<Vec<u8>>,
    storage: HashMap<U256, Vec<u8>>,
}

#[derive(Debug)]
pub struct Blockchain<'a> {
    blocks: Vec<Block>,
    accounts: HashMap<&'a str, Account>,
}

impl<'a> Blockchain<'a> {
    pub fn new() -> Blockchain<'a> {
        let gen = Block::new(0, Hash::zero());
        let mut accounts = HashMap::new();
        accounts.insert("alice", Account::new(Address::random(), 1000000));
        accounts.insert("bob", Account::new(Address::random(), 1000000));
        accounts.insert("carol", Account::new(Address::random(), 1000000));
        accounts.insert("dave", Account::new(Address::random(), 1000000));

        Blockchain {
            blocks: vec![gen],
            accounts: accounts,
        }
    }

    pub fn commit(&mut self) {
        let block = Block::new(self.blocks.len() as u32, self.blocks.last().unwrap().hash());

        self.blocks.push(block);
    }

    pub fn get_address(&self, alias: &str) -> Address {
        self.accounts.get(alias).unwrap().address
    }
}

impl<'a> StateProvider for Blockchain<'a> {
    fn storage_at(&self, key: U256) -> U256 {
        U256::zero()
    }
    fn blockhahs(&self, num: i64) -> U512 {
        U512::zero()
    }
    fn exist(&self, address: Address) -> bool {
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
    pub fn new(addr: Address, bal: u64) -> Account {
        Account {
            address: addr,
            balance: bal,
            code: None,
            storage: HashMap::new(),
        }
    }
}
