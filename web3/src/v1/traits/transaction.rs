use crate::v1;
use ethereum_types::{H160, H256, H520, U256};
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;
use v1::types::{TransactionRequest,TxReceipt};
use v1::types::Bytes;
use common_types::{BlockNumber};


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
	fn getTransaction_Receipt(&self,_:H160 ) -> Result<TxReceipt>;

    #[rpc(name = "eth_getCode")]
	fn code_at(&self, _: H160, _: Option<BlockNumber>) -> Result<H160>;


}
