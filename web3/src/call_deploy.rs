extern crate blockchain;
extern crate durian;
use crate::v1;
use blockchain::blockchain::Blockchain;
use durian::stateless_vm::StatelessVM;
use durian::transaction::Transaction;
use v1::types::TransactionRequest;
use ethereum_types::{Address, H160, H256, U256, U512};

pub fn call_deploy(request: TransactionRequest) {
	let mut bc = Blockchain::new();
	let from = request.from.unwrap();
	let data = request.data.unwrap();
	let gas_price = request.gas_price.unwrap();
	let gas = request.gas.unwrap();
	let mut code = Vec::new();
	bc.commit();
	code = data.into_vec();

	let params1 = vec![
		0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
		0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
		0xFF, 0xFF,
	];

	let tx1 = Transaction::create(from,U256::zero(),U256::from(1000000),code, params1);
	let vm = StatelessVM::new();
	let ret1 = vm.fire(tx1, &mut bc).unwrap();
	bc.incNonce("naga");
	bc.commit();
}
