use ethereum_types::{H256};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub num: u64,
    pub prev: H256,
    pub time: SystemTime,
}

impl Block {
    pub fn new(num: u64, prev: H256) -> Block {
        Block {
            num: num,
            prev: prev,
            time: std::time::SystemTime::now(),
        }
    }

    pub fn hash(&self) -> H256 {
        let bytes = bincode::serialize(self).unwrap();
        H256::from_slice(Keccak256::digest(&bytes).as_slice())
    }

    pub fn block_number(&self) -> u64 {
        self.num
    }
}
