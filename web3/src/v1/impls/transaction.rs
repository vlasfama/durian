use crate::v1;
use ethereum_types::{Address, H160, H256,H520, U256, U512};
use jsonrpc_core::futures::{future, Future};
use jsonrpc_core::types::Value;
use jsonrpc_core::{Error,Params};
use jsonrpc_core::{BoxFuture, Result};
use v1::helpers::errors;
use v1::metadata::Metadata;
use v1::traits::TransactionRPC;
use v1::types::TransactionRequest;
pub struct TransactionRPCImpl;
use crate::call_deploy::call_deploy;

use std::io;
impl TransactionRPCImpl {
	pub fn new() -> Self {
		TransactionRPCImpl {}
	}
}

impl TransactionRPC for TransactionRPCImpl {
	type Metadata = Metadata;

	fn gas_price(&self) -> BoxFuture<U256> {
		let trx_count = U256::zero();
		println! {"the trx_count {:?}",trx_count}
		let result = Ok(trx_count);
		Box::new(future::done(result))
	}

	fn send_transaction(&self, request: TransactionRequest) -> Result<H160> {
		let result = call_deploy(request);
		Ok((result.unwrap()))
	}

	fn getTransaction_Receipt(&self, hash:H520) -> Result<H160> {
		// let result=call_deploy(request);
		Ok(H160::zero())
	}
}
