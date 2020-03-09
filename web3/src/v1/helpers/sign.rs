
use crate::v1;
extern crate durian;
use v1::types::{SignedTransaction};
use durian::transaction::{Transaction,Action};
use parity_bytes::Bytes;



use std::cmp::min;

use ethereum_types::U256;
use jsonrpc_core::Error;
use v1::helpers::request::CallRequest;

// pub fn sign_call(request: CallRequest) -> Result<SignedTransaction, Error> {
// 	let max_gas = U256::from(500_000_000);
// 	let gas = min(request.gas.unwrap_or(max_gas), max_gas);
// 	let from = request.from.unwrap_or_default();
// 	Ok(Transaction {
// 		action: request.to.map_or(Action::Create, Action::Call),
// 		gas: request.gas_price.unwrap_or_default(),
// 		value: request.value.unwrap_or_default(),
// 		code: request.data.unwrap_or_default(),
// 		sender:from,
// 		params:vec![]
// 	}.fake_sign(from))
// }




