use crate::v1;
use ethereum_types::{H160, H256, U256};
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;
use v1::types::CallRequest;
use v1::types::{BlockNumber, Bytes};
use v1::types::{TransactionRequest, TxReceipt};

#[rpc(server)]
pub trait TransactionRPC {
	/// RPC Metadata
	type Metadata;

	/// Returns current gas_price.
	#[rpc(name = "eth_gasPrice")]
	fn gas_price(&self) -> BoxFuture<U256>;

	#[rpc(name = "eth_sendTransaction")]
	fn send_transaction(&self, tx: TransactionRequest) -> Result<H256>;

	#[rpc(name = "eth_getTransactionReceipt")]
	fn transaction_receipt(&self, _: H256) -> Result<TxReceipt>;

	#[rpc(name = "eth_getCode")]
	fn code_at(&self, _: H160, _: Option<BlockNumber>) -> Result<Bytes>;

	/// Call contract, returning the output data.
	#[rpc(name = "eth_call")]
	fn call(&self, call: CallRequest, bn: Option<BlockNumber>) -> Result<Bytes>;
}
