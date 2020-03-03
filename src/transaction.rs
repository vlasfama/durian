use ethereum_types::{Address, U256};
use parity_bytes::Bytes;


#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Create,
    Call(Address),
}

#[derive(Debug)]
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


}
