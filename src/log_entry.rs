use ethereum_types::{Address, H256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogEntry {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}
