use crate::v1;
use ethereum_types::{H160, H256, U256};
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;
use v1::types::{BlockNumber, Bytes};
use v1::types::{CallRequest, TransactionRequest, TxReceipt};

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
