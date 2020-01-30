extern crate bincode;
extern crate ethereum_types;
extern crate hex;
extern crate serde;
extern crate serde_derive;
extern crate sha3;
extern crate time;

use durian::state_provider::{StateAccount, StateProvider};
use durian::error::Error;
use ethereum_types::{Address, H160, H256, U256, U512};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use time::PrimitiveDateTime;
use jsonrpc_core::types::params::Params;

pub type Hash = H256;
use jsonrpc_http_server::jsonrpc_core::Params;
use serde_json::{json, Value};
use std::fmt;
use std::fmt::Formatter;
extern crate hex_slice;
use durian::stateless_vm::StatelessVM;
use hex_slice::AsHex;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Block {
    num: u32,
    prev: Hash,
    time: PrimitiveDateTime,
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
pub struct Blockchain {
    blocks: Vec<Block>,
    accounts: HashMap<String, Account>,
    counter: i32,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let gen = Block::new(0, Hash::zero());
        let mut accounts = HashMap::new();
        let addr = Blockchain::add_account();
        accounts.insert(
            "alice".to_string(),
            Account::new(Address::random(), U256::from(1000000), U256::zero()),
        );
        accounts.insert(
            "bob".to_string(),
            Account::new(Address::random(), U256::from(1000000), U256::zero()),
        );
        accounts.insert(
            "carol".to_string(),
            Account::new(Address::random(), U256::from(1000000), U256::zero()),
        );
        accounts.insert(
            "dave".to_string(),
            Account::new(Address::random(), U256::from(1000000), U256::zero()),
        );
        accounts.insert(
            "naga".to_string(),
            Account::new(addr, U256::from(1000000), U256::zero()),
        );

        Blockchain {
            blocks: vec![gen],
            accounts: accounts,
            counter: 0,
        }
    }

    pub fn commit(&mut self) {
        let block = Block::new(self.blocks.len() as u32, self.blocks.last().unwrap().hash());

        info!("Committing new block. num: {:?}, hash: {:?}", block.num, block.hash());
        self.blocks.push(block);

        info!("Accounts:");
        for (alias, acc) in self.accounts.iter() {
            info!("  {:?}: address: {:?}, balance: {:?}, nonce:{:?}", alias, acc.address, acc.balance, acc.nonce);

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

    pub fn code(&self, alias: &str) -> Vec<u8> {
        self.accounts.get(alias).unwrap().code.clone()
    }

    pub fn incNonce(& mut self, alias: &str) {
        let mut acc  = self.accounts.get_mut(alias).unwrap();
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

    pub fn call_contract(& mut self, contract_value: Params) {

        let mut data : &str=
        match contract_value {
			Params::Array(ref vec) => vec[0].as_str().unwrap(),
			Params::Map(map) => panic!("Invalid return data"),
			Params::None => panic!("Invalid return data"),
        };

        let v: Value = serde_json::from_str(data).unwrap();

        println!("the value in v {:?}", v);

        let from_address = v["_parent"]["defaultAccount"].clone();
        let from = from_address.as_str().unwrap();
        //let address_hex = from.trim_start_matches("0x");
        let address_hex = &from[2..];
        let address_bs = hex::decode(address_hex).expect("Decoding failed");
        let address = Address::from_slice(&address_bs);

        // let dd = v["_deployData"].clone();
        let dd = v["_parent"]["options"]["data"].clone();
        let deploy_data = dd.as_str().unwrap();
        let dd_hex = &deploy_data[2..];
        let dd_bs = hex::decode(dd_hex).expect("Decoding failed");


        println!("the value in deploy_data_bytes###################################################### {:?}", dd_bs);

        let tx1 = Transaction::create(address, U256::zero(), U256::from(10000000), dd_bs, vec![]);

        let vm = StatelessVM::new();
        let ret_1 = vm.fire(tx1, self);
    }

    pub fn add_account() -> H160 {
        let from = "0x004ec07d2329997267ec62b4166639513386f32e";
        //let address_hex = from.trim_start_matches("0x");
        let address_hex = &from[2..];
        let address_bs = hex::decode(address_hex).expect("Decoding failed");
        let address = Address::from_slice(&address_bs);
        return address;
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

    fn create_contract(&mut self, address: Address, nonce: U256) {
        let name = format!("contract_{}", self.counter+1);
        let mut acc = Account::new(address, U256::zero(), nonce);
        self.accounts.insert(name, acc);

        self.counter = self.counter + 1;
    }

    fn storage_at(&self, address: &Address, key: &H256) -> Result<H256, Error> {
        let acc = self.account(address)?;
        match acc.storage.get(key) {
            Some(storage) => Ok(*storage),
            _ => Err(Error::InvalidStorageKey),
        }
    }
    fn set_storage(&mut self, address: &Address, key: &H256, value: &H256) {
        for (_, acc) in self.accounts.iter_mut() {
            if acc.address == *address {
                let val = acc.storage.entry(*key).or_insert(*value);
                *val = *value;
                break;
            }
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

    fn init_code(&mut self, address: &Address, code: Vec<u8>) {
        for (_, acc) in self.accounts.iter_mut() {
            if acc.address == *address {
                acc.code = code;
                break;
            }
        }
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
}

impl Account {
    pub fn new(addr: Address, bal: U256, nonce: U256) -> Account {
        Account {
            address: addr,
            balance: bal,
            nonce: nonce,
            code: vec![],
            storage: HashMap::new(),
        }
    }
}
