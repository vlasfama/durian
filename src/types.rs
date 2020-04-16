use super::Bytes;
use ethereum_types::{Address, H256, U256};

/// The type of the instruction.
#[derive(Debug, PartialEq, Clone)]
pub enum ActionType {
    /// CREATE.
    Create,
    /// CALL.
    Call,
    // TODO:::::::: do we need create2,...
    // CALLCODE.
    ///////////////// CallCode,
    ///////////////// /// DELEGATECALL.
    ///////////////// DelegateCall,
    ///////////////// /// STATICCALL.
    ///////////////// StaticCall
    ///////////////// // TODO:::::::: do we need create2,
    ///////////////// /// CREATE2.
    ///////////////// /// Create2,
}

/// Action (call/create) input params. Everything else should be specified in Externalities.
#[derive(Clone, Debug)]
pub struct ActionParams {
    /// Address of currently executed code.
    pub code_address: Address,
    /// Hash of currently executed code.
    pub code_hash: Option<H256>,
    /// Receive address. Usually equal to code_address,
    /// except when called using CALLCODE.
    pub address: Address,
    /// Sender of current part of the transaction.
    pub sender: Address,
    /// Transaction initiator.
    pub origin: Address,
    /// Gas paid up front for transaction execution
    pub gas: U256,
    /// Gas price.
    pub gas_price: U256,
    /// Transaction value.
    pub value: U256,
    /// Code being executed.
    pub code: Bytes,
    /// Code version being executed.
    pub code_version: U256,
    /// Arguments
    pub args: Bytes,
    /// Type of action (e.g. CALL, DELEGATECALL, CREATE, etc.)
    pub action_type: ActionType,
}
