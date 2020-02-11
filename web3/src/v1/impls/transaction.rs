use crate::v1;
use ethereum_types::{Address, H160, H256, U256, U512};
use jsonrpc_core::futures::{future, Future};
use jsonrpc_core::types::Value;
use jsonrpc_core::Error;
use jsonrpc_core::{BoxFuture, Result};
use v1::helpers::errors;
use v1::traits::TransactionRPC;
use v1::types::TransactionRequest;


pub struct TransactionRPCImpl;


impl TransactionRPCImpl {
	pub fn new() -> Self {
		TransactionRPCImpl {}
	}
}


impl TransactionRPC for TransactionRPCImpl {

	fn gas_price(&self) -> BoxFuture<U256> {
		let trx_count = U256::zero();
		let result = Ok(trx_count);
        Box::new(future::done(result))

        // // Some(U256::zero());
        // // return Err()
        //  Ok(U256::zero())
	}


}