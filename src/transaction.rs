use ethereum_types::{Address, U256};
use parity_bytes::Bytes;
use std::sync::Arc;

#[derive(Debug)]
pub struct Transaction {
    pub caller: Address,
    pub contract: Address,
    pub gas: U256,
    pub code: Bytes,
    pub data: Bytes,
}

impl Transaction {
    pub fn new(
        caller: Address,
        contract: Address,
        gas: U256,
        code: Bytes,
        data: Bytes,
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
