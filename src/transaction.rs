use ethereum_types::{Address, U256};
use parity_bytes::Bytes;
use std::sync::Arc;

#[derive(Debug)]
pub struct Transaction {
    pub caller: Address,
    pub contract: Address,
    pub gas: U256,
    pub code: Option<Arc<Bytes>>,
    pub data: Option<Bytes>,
}

impl Transaction {
    pub fn new(
        caller: Address,
        contract: Address,
        gas: U256,
        code: Option<Arc<Bytes>>,
        data: Option<Bytes>,
    ) -> Self {
        Transaction {
            caller,
            contract,
            gas,
            code,
            data,
        }
    }
}
