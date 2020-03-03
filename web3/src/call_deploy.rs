extern crate blockchain;
extern crate durian;
use crate::v1;
use blockchain::blockchain::Blockchain;
use durian::stateless_vm::StatelessVM;
use durian::transaction::Transaction;
use ethereum_types::{Address, H160, H256, H520, U256};
use jsonrpc_core::futures::{future, Future};
use jsonrpc_core::{BoxFuture, Params};
use std::io;
use v1::helpers::errors;
use v1::types::{TransactionRequest, TxReceipt};

pub fn call(bc: &mut Blockchain, request: TransactionRequest) -> Result<H160, io::Error> {
	let mut bc = bc;
	let vm = StatelessVM::new();
	let from = request.from.unwrap();
	let data = request.data.unwrap();
	let value = request.value.unwrap_or(U256::zero());
	let gas_price = request.gas_price.unwrap();
	let gas = request.gas.unwrap();
	let code = data.into_vec();
	bc.commit();
	let tx1 = Transaction::create(from, value, gas, code, vec![]);
	let ret1 = vm.fire(tx1, bc).unwrap();
	let contract_address = ret1.contract;
	bc.incNonce("naga");
	bc.commit();
	Ok((contract_address))
}

pub fn transaction_recipit(bc: &mut Blockchain, params: H160) -> Result<TxReceipt, io::Error> {
	let h1 = H256::zero();
	let u1 = U256::zero();
	let receipt = TxReceipt {
		transaction_hash: h1,
		transaction_index: u1,
		block_hash: bc.last_block_hash(),
		from: bc.address("naga"),
		to: H160::zero(),
		block_number: u1,
		gas_used: u1,
		contract_address: params,
	};

	Ok((receipt))
}
