extern crate blockchain;
extern crate durian;
use crate::v1;
use blockchain::blockchain::Blockchain;
use durian::stateless_vm::StatelessVM;
use durian::transaction::Transaction;
use ethereum_types::{H160, H256, U256};
use jsonrpc_core::futures::{future, Future};
use jsonrpc_core::{BoxFuture};
use v1::helpers::errors;
use v1::types::TransactionRequest;
use std::io;


pub fn call_deploy(request: TransactionRequest) -> Result<H160,io::Error> {
	let mut bc = Blockchain::new();
	let from = request.from.unwrap();
	let data = request.data.unwrap();
	let value = request.value.unwrap_or(U256::zero());
	let gas_price = request.gas_price.unwrap();
	let gas = request.gas.unwrap();
	let code = data.into_vec();
	bc.commit();
	let tx1 = Transaction::create(from, value, gas, code, vec![]);
	let vm = StatelessVM::new();
	let ret1 = vm.fire(tx1, &mut bc).unwrap();
	println!("ret1: {:?}", ret1);
	println!("ret1: {:?}", ret1.contract);
	let contract_address = ret1.contract;
	bc.incNonce("naga");
	bc.commit();
	Ok((contract_address))

}
