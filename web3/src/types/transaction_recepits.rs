use ethereum_types::{Address, H160, H256, U256};
use serde_derive::{Deserialize, Serialize};

/// Transaction request coming from RPC
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TxReceipt {
    /// Transaction Hash
    pub transaction_hash: H256,
    /// Transaction index
    pub transaction_index: U256,
    /// Block hash
    pub blockHash: H256,
    /// Sender
    pub from: Address,
    /// Recipient
    pub to: H160,
    /// Block number
    pub blockNumber: u32,
    /// Cumulative gas used
    /// Gas used
    pub gasUsed: U256,
    /// Contract address
    pub contractAddress: H160,
    /// Cumulative gas used
    pub cumulativeGasUsed: U256,

    pub status: &'static str,
}
