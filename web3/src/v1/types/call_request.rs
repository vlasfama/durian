use crate::v1;
use ethereum_types::{H160, U256};
use serde_derive::{Deserialize, Serialize};
use v1::helpers::request::CallRequest as Request;
use v1::types::Bytes;

/// Call request
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CallRequest {
	/// From
	pub from: Option<H160>,
	/// To
	pub to: Option<H160>,
	/// Gas Price
	pub gasPrice: Option<U256>,
	/// Gas
	pub gas: Option<U256>,
	/// Value
	pub value: Option<U256>,
	/// Data
	pub data: Option<Bytes>,
	/// Nonce
	pub nonce: Option<U256>,
}

impl Into<Request> for CallRequest {
	fn into(self) -> Request {
		Request {
			from: self.from.map(Into::into),
			to: self.to.map(Into::into),
			gas_price: self.gasPrice.map(Into::into),
			gas: self.gas.map(Into::into),
			value: self.value.map(Into::into),
			data: self.data.map(Into::into),
			nonce: self.nonce.map(Into::into),
		}
	}
}
