extern crate bincode;
extern crate ethereum_types;
extern crate serde;
extern crate serde_derive;
extern crate sha3;

use durian::state_provider::{StateProvider, Account as StateAccount};
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
    code: Option<Vec<u8>>,
    storage: HashMap<U256, Vec<u8>>,
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
        accounts.insert("alice", Account::new(Address::random(), U256::from(1000000)));
        accounts.insert("bob", Account::new(Address::random(), U256::from(1000000)));
        accounts.insert("carol", Account::new(Address::random(), U256::from(1000000)));
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

    pub fn get_address(&self, alias: &str) -> Address {
        self.accounts.get(alias).unwrap().address
    }
}

impl<'a> StateProvider for Blockchain<'a> {
    fn get_account(&self, address: Address) -> Option<StateAccount> {


        for (k, v) in self.accounts.iter() {
            if v.address == address {
                return Some(StateAccount{
                    balance: U256::from(v.balance),
                    nonce: U256::from(v.nonce),
                    code: v.code.clone(), // TODO:: better way?????
                })
            }
        }

        None

    }

    fn create_account(&mut self, address: Address, account: StateAccount) {


        //let name = format!("contract_{}", self.counter);
        let mut acc = Account::new(address, account.balance);
        acc.code = account.code;
        //self.accounts.insert(&name, acc);
        self.accounts.insert("contract", acc);
        self.counter =self.counter+1 ;
    }


    fn storage_at(&self, key: U256) -> U256 {
        U256::zero()
    }
    fn blockhash(&self, num: i64) -> U512 {
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
    pub fn new(addr: Address, bal: U256) -> Account {
        Account {
            address: addr,
            balance: bal,
            nonce: U256::from(0),
            code: None,
            storage: HashMap::new(),
        }
    }
}
