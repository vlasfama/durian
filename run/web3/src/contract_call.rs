extern crate blockchain;
extern crate durian;
use crate::types;
use blockchain::blockchain::Blockchain;
use durian::execute;
use durian::transaction::Transaction;
use ethereum_types::{H160, H256, U256};
use std::io;
use types::{TransactionRequest, TxReceipt};

//Deploy the contract
pub fn create(bc: &mut Blockchain, request: TransactionRequest) -> Result<H256, io::Error> {
	let from = request.from.unwrap();
	let data = request.data.unwrap();
	let value = request.value.unwrap_or(U256::zero());
	let gas = request.gas.unwrap();
	let code = data.into_vec();
	bc.commit();
	let tx1 =
		Transaction::make_create_embedded_code(from, value, gas, U256::zero(), code, H256::zero());
	let ret1 = execute::execute(&tx1.clone(), bc).unwrap();
	let tx_hash = bc.add_transactions(tx1, ret1);
	bc.inc_nonce("naga");
	bc.commit();
	Ok(tx_hash)
}

//Generate the transaction recepit
pub fn transaction_recipit(bc: &mut Blockchain, params: H256) -> Result<TxReceipt, io::Error> {
	let tx_details = bc.get_transaction_details(params);
	let sender = tx_details.clone().unwrap().0.sender;
	let contract_address = tx_details.clone().unwrap().1.contract;
	let gas_left = tx_details.clone().unwrap().1.gas_left;
	let u1 = U256::zero();
	println!("the bc contains {:?}", bc);

	let receipt = TxReceipt {
		transaction_hash: params,
		blockHash: bc.latest_block_hash(),
		from: sender,
		to: H160::zero(),
		blockNumber: bc.block_number() as u32,
		gasUsed: gas_left,
		contractAddress: contract_address,
		cumulativeGasUsed: gas_left,
		transaction_index: u1,
		status: "0x01",
	};
	Ok(receipt)
}
