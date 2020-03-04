use crate::contract_call::{create, transaction_recipit};
use crate::v1;
use blockchain::blockchain::Blockchain;
use ethereum_types::{Address, H160, H256, H520, U256};
use jsonrpc_core::futures::future;
use jsonrpc_core::{BoxFuture, Result};
use std::sync::{Arc, Mutex};
use v1::metadata::Metadata;
use v1::traits::TransactionRPC;
use v1::types::{Bytes,BlockNumber};
use v1::types::{TransactionRequest, TxReceipt};
use std::fmt;

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

	//get the gas_price
	fn gas_price(&self) -> BoxFuture<U256> {
		let trx_count = U256::zero();
		let result = Ok(trx_count);
		Box::new(future::done(result))
	}

	//create the contract deployment transaction
	fn send_transaction(&self, request: TransactionRequest) -> Result<H256> {
		let mut bc = self.bc.lock().unwrap();
		let result = create(&mut bc, request);
		Ok(result.unwrap())
	}

	//Generate the contract transcation recepit
	fn transaction_receipt(&self, params: H256) -> Result<TxReceipt> {
		let mut bc = self.bc.lock().unwrap();
		let tx_recipt = transaction_recipit(&mut bc, params);
		Ok(tx_recipt.unwrap())
	}

	//Get the contract code address from the blockchain
	fn code_at(&self, address: H160, num: Option<BlockNumber>) -> Result<Bytes> {
		let mut bc = self.bc.lock().unwrap();
		let code_at = &bc.code_at(address);
		let res = Bytes::new(code_at.to_vec());
		Ok(res)
	}
}
