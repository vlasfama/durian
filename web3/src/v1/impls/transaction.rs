use crate::call_deploy::{call, transaction_recipit};
use crate::v1;
use blockchain::blockchain::Blockchain;
use common_types::BlockNumber;
use ethereum_types::{H160, H256, H520, U256};
use jsonrpc_core::futures::future;
use jsonrpc_core::{BoxFuture, Result};
use std::sync::{Arc, Mutex};
use v1::metadata::Metadata;
use v1::traits::TransactionRPC;
use v1::types::Bytes;
use v1::types::{TransactionRequest, TxReceipt};

pub struct TransactionRPCImpl {
	bc: Mutex<Blockchain>,
}

impl TransactionRPCImpl {
	pub fn new(bc: Blockchain) -> Self {
		TransactionRPCImpl { bc: Mutex::new(bc) }
	}
}

impl TransactionRPC for TransactionRPCImpl {
	type Metadata = Metadata;

	fn gas_price(&self) -> BoxFuture<U256> {
		let trx_count = U256::zero();
		let result = Ok(trx_count);
		Box::new(future::done(result))
	}

	fn send_transaction(&self, request: TransactionRequest) -> Result<H160> {
		let mut bc = self.bc.lock().unwrap();
		let result = call(&mut bc, request);
		Ok(result.unwrap())
	}

	fn getTransaction_Receipt(&self, params: H160) -> Result<TxReceipt> {
		let mut bc = self.bc.lock().unwrap();
		let tx_recipt = transaction_recipit(&mut bc, params);
		Ok((tx_recipt.unwrap()))
	}

	fn code_at(&self, address: H160, num: Option<BlockNumber>) -> Result<H160> {
		// let address: Address = H160::into(address);

		// let num = num.unwrap_or_default();
		// try_bf!(check_known(&*self.client, num.clone()));

		// let res = match self.client.code(&address, self.get_state(num)) {
		// 	StateResult::Some(code) => Ok(code.map_or_else(Bytes::default, Bytes::new)),
		// 	StateResult::Missing => Err(errors::state_pruned()),
		// };

		Ok(H160::zero())
	}
}
