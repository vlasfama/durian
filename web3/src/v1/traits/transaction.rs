use crate::v1;
use ethereum_types::{H160, H256, H520, U256};
use jsonrpc_core::{Error,Params};
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;
use v1::types::TransactionRequest;
#[rpc(server)]
pub trait TransactionRPC {
	/// RPC Metadata
	type Metadata;

	/// Returns current gas_price.
	#[rpc(name = "eth_gasPrice")]
	fn gas_price(&self) -> BoxFuture<U256>;

	#[rpc(name = "eth_sendTransaction")]
	fn send_transaction(&self, tx: TransactionRequest) -> Result<H160>;

	#[rpc(name = "eth_getTransactionReceipt")]
	fn getTransaction_Receipt(&self, tx: H520) -> Result<H160>;


}
