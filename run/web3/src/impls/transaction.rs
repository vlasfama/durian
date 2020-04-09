extern crate durian;
use crate::contract_call::{create, transaction_recipit};
use crate::metadata;
use crate::traits;
use crate::types;
use blockchain::blockchain::Blockchain;
use durian::execute;
use durian::transaction::Transaction;
use ethereum_types::{H160, H256, U256};
use jsonrpc_core::futures::future;
use jsonrpc_core::{BoxFuture, Result};
use metadata::Metadata;
use std::sync::Mutex;
use traits::TransactionRPC;
use types::{BlockNumber, Bytes};
use types::{CallRequest, TransactionRequest, TxReceipt};

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

	fn call(&self, request: CallRequest, num: Option<BlockNumber>) -> Result<Bytes> {
		let mut bc = self.bc.lock().unwrap();

		let contract_address = request.to.unwrap();
		let params = request.data.unwrap();
		let params_vec = Bytes::into_vec(params);

		let tx_call = Transaction::make_call(
			bc.address("naga"),
			contract_address,
			U256::zero(),
			U256::from(1000000),
			U256::zero(),
			params_vec,
		);

		let ret3 =execute::execute(&tx_call.clone(), &mut *bc).unwrap();
		println!("the value inside ret3 {:?}", ret3);
		let res = Bytes::new(ret3.data);
		Ok(res)
	}
}
