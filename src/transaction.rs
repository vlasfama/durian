use ethereum_types::{Address,H256, U256};
use parity_bytes::Bytes;
use parity_util_mem::MallocSizeOf;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};


pub type Hash = H256;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, MallocSizeOf, Eq)]
pub enum Action {
    Create,
    Call(Address),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, MallocSizeOf, Eq)]
pub struct Transaction {
    pub sender: Address,
    pub value: U256,
    pub gas: U256,
    pub action: Action,
    pub code: Bytes,
    pub params: Bytes,
}

impl Transaction {
    pub fn create(sender: Address, value: U256, gas: U256, code: Bytes, params: Bytes) -> Self {
        Transaction {
            action: Action::Create,
            sender,
            value,
            gas,
            code,
            params,
        }
    }

    pub fn call(
        sender: Address,
        contract: Address,
        value: U256,
        gas: U256,
        code: Bytes,
        params: Bytes,
    ) -> Self {
        Transaction {
            action: Action::Call(contract),
            sender,
            value,
            gas,
            code,
            params,
        }
    }

    pub fn new(sender: Address, value: U256, gas: U256, code: Bytes, params: Bytes) -> Self {
        Transaction {
            action: Action::Create,
            sender,
            value,
            gas,
            code,
            params,
        }
    }

    pub fn hash(&self) -> Hash {
        let bytes = bincode::serialize(self).unwrap();
        Hash::from_slice(Keccak256::digest(&bytes).as_slice())
    }


}
