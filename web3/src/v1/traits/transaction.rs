
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;
use crate::v1;
use ethereum_types::{H160, U256,H256, H520};
use v1::types::{TransactionRequest};

#[rpc]
pub trait TransactionRPC {
	#[rpc(name = "eth_sendTransaction")]
	fn send_transaction(&self,t: TransactionRequest) -> Result<H256>;

	/// Returns current gas_price.
	#[rpc(name = "eth_gasPrice")]
	fn gas_price(&self) -> Result<U256>;
}