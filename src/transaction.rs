use super::Bytes;
use ethereum_types::{Address, H256, U256};
use parity_wasm::peek_size;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    /// Create creates new contract.
    /// Code + salt
    Create(Bytes, H256),
    /// Calls contract at given address.
    /// In the case of a transfer, this is the receiver's address.'
    Call(Address),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub sender: Address,
    pub value: U256,
    pub gas: U256,
    pub gas_price: U256,
    pub action: Action,
    pub args: Bytes,
}

impl Transaction {
    pub fn make_create_embedded_code(
        sender: Address,
        value: U256,
        gas: U256,
        gas_price: U256,
        code_params: Bytes,
        salt: H256,
    ) -> Self {
        let module_size = peek_size(&*code_params);
        let code = code_params[..module_size].to_vec();
        let args = code_params[module_size..].to_vec();

        Transaction {
            action: Action::Create(code, salt),
            sender,
            value,
            gas,
            gas_price,
            args,
        }
    }

    pub fn make_create(
        sender: Address,
        value: U256,
        gas: U256,
        gas_price: U256,
        code: Bytes,
        args: Bytes,
        salt: H256,
    ) -> Self {
        Transaction {
            action: Action::Create(code, salt),
            sender,
            value,
            gas,
            gas_price,
            args,
        }
    }

    pub fn make_call(
        sender: Address,
        contract: Address,
        value: U256,
        gas: U256,
        gas_price: U256,
        args: Bytes,
    ) -> Self {
        Transaction {
            action: Action::Call(contract),
            sender,
            value,
            gas,
            gas_price,
            args,
        }
    }

    // TODO: move this function out of transaction
    pub fn hash(&self) -> H256 {
        let bytes = bincode::serialize(self).unwrap();
        H256::from_slice(Keccak256::digest(&bytes).as_slice())
    }
}
