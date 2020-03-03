use ethereum_types::{Address, H160, H256, H520, U256};
use serde_derive::{Deserialize, Serialize};
use serde_json;
/// Transaction request coming from RPC
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TxReceipt {
    /// Transaction Hash
    pub transaction_hash:H256,
    /// Transaction index
    pub transaction_index:U256,
    /// Block hash
    pub block_hash: H256,
    /// Sender
    pub from: Address,
    /// Recipient
    pub to: H160,
    /// Block number
    pub block_number: U256,
    /// Cumulative gas used
    /// Gas used
    pub gas_used:U256,
    /// Contract address
    pub contract_address:H160,
}
