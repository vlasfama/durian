use crate::types;
use ethereum_types::{H160, U256};
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use types::BlockNumber;
use types::CallRequest;

#[rpc(server)]
pub trait AccountRPC {
    /// RPC Metadata
    type Metadata;

    /// Returns balance of the given account.
    #[rpc(name = "eth_getBalance")]
    fn balance(&self, _: H160, _: Option<BlockNumber>) -> Result<U256>;

    /// Estimate gas needed for execution of given contract.
    #[rpc(name = "eth_estimateGas")]
    fn estimate_gas(&self, _: CallRequest, _: Option<BlockNumber>) -> Result<U256>;
}
