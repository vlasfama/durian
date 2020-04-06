use ethereum_types::{Address, H256, U256};
use super::Bytes;

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

/// Gas Left: either it is a known value, or it needs to be computed by processing
/// a return instruction.
#[derive(Debug)]
pub enum GasLeft {
    /// Known gas left
    Known(U256),
    /// Return or Revert instruction must be processed.
    NeedsReturn {
        /// Amount of gas left.
        gas_left: U256,
        /// Return data buffer.
        data: ReturnData,
        /// Apply or revert state changes on revert.
        apply_state: bool,
    },
}

/// Return data buffer. Holds memory from a previous call and a slice into that memory.
#[derive(Debug)]
pub struct ReturnData {
    mem: Vec<u8>,
    offset: usize,
    size: usize,
}

impl ReturnData {
	pub fn new(mem: Vec<u8>, offset: usize, size: usize) -> Self {
		ReturnData { mem, offset, size }
	}
}

#[derive(Debug)]
/// Result of externalities create function.
pub enum ContractCreateResult {
    /// Returned when creation was successfull.
    /// Contains an address of newly created contract and gas left.
    Created(Address, U256),
    /// Returned when contract creation failed.
    /// VM doesn't have to know the reason.
    Failed,
    /// Reverted with REVERT.
    Reverted(U256, ReturnData),
}

#[derive(Debug)]
/// Result of externalities call function.
pub enum MessageCallResult {
    /// Returned when message call was successfull.
    /// Contains gas left and output data.
    Success(U256, ReturnData),
    /// Returned when message call failed.
    /// VM doesn't have to know the reason.
    Failed,
    /// Returned when message call was reverted.
    /// Contains gas left and output data.
    Reverted(U256, ReturnData),
}

#[derive(Debug)]
pub enum TrapKind {
    Call(ActionParams),
    Create(ActionParams, Address),
}
