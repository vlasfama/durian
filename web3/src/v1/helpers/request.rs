
use ethereum_types::{U256, H256, Address};
use crate::v1;
use v1::types::{Bytes};



/// Transaction request coming from RPC
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct TransactionRequest {
	/// Sender
	pub from: Option<Address>,
	/// Recipient
	pub to: Option<Address>,
	/// Gas Price
	pub gas_price: Option<U256>,
	/// Gas
	pub gas: Option<U256>,
	/// Value of transaction in wei
	pub value: Option<U256>,
	/// Additional data sent with transaction
	pub data: Option<Bytes>,
	/// Transaction's nonce
	pub nonce: Option<U256>


}


/// Transaction request coming from RPC with default values filled in.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct FilledTransactionRequest {
	/// Sender
	pub from: Address,
	/// Indicates if the sender was filled by default value.
	pub used_default_from: bool,
	/// Recipient
	pub to: Option<Address>,
	/// Gas Price
	pub gas_price: U256,
	/// Gas
	pub gas: U256,
	/// Value of transaction in wei
	pub value: U256,
	/// Additional data sent with transaction
	pub data: Bytes,
	/// Transaction's nonce
	pub nonce: Option<U256>

}

impl From<FilledTransactionRequest> for TransactionRequest {
	fn from(r: FilledTransactionRequest) -> Self {
		TransactionRequest {
			from: Some(r.from),
			to: r.to,
			gas_price: Some(r.gas_price),
			gas: Some(r.gas),
			value: Some(r.value),
			data: Some(r.data),
			nonce: r.nonce,

		}
	}
}