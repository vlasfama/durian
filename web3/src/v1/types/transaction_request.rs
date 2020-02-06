use ethereum_types::{H160, U256};
use crate::v1;
use v1::types::{Bytes};
use std::fmt;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use ansi_term::Colour;
use v1::helpers;
use helpers::FilledTransactionRequest;

/// Transaction request coming from RPC
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
	/// Sender
	pub from: Option<H160>,
	/// Recipient
	pub to: Option<H160>,
	/// Gas Price
	pub gas_price: Option<U256>,
	/// Gas
	pub gas: Option<U256>,
	/// Value of transaction in wei
	pub value: Option<U256>,
	/// Additional data sent with transaction
	pub data: Option<Bytes>,
	/// Transaction's nonce
	pub nonce: Option<U256>,

}

pub fn format_ether(i: U256) -> String {
	let mut string = format!("{}", i);
	let idx = string.len() as isize - 18;
	if idx <= 0 {
		let mut prefix = String::from("0.");
		for _ in 0..idx.abs() {
			prefix.push('0');
		}
		string = prefix + &string;
	} else {
		string.insert(idx as usize, '.');
	}
	String::from(string.trim_end_matches('0').trim_end_matches('.'))
}

impl fmt::Display for TransactionRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let eth = self.value.unwrap_or_default();
		match self.to {
			Some(ref to) => write!(
				f,
				"{} ETH from {} to 0x{:?}",
				Colour::White.bold().paint(format_ether(eth)),
				Colour::White.bold().paint(
					self.from.as_ref()
						.map(|f| format!("0x{:?}", f))
						.unwrap_or_else(|| "?".to_string())),
				to
			),
			None => write!(
				f,
				"{} ETH from {} for contract creation",
				Colour::White.bold().paint(format_ether(eth)),
				Colour::White.bold().paint(
					self.from.as_ref()
						.map(|f| format!("0x{:?}", f))
						.unwrap_or_else(|| "?".to_string())),
			),
		}
	}
}

impl From<helpers::TransactionRequest> for TransactionRequest {
	fn from(r: helpers::TransactionRequest) -> Self {
		TransactionRequest {
			from: r.from.map(Into::into),
			to: r.to.map(Into::into),
			gas_price: r.gas_price.map(Into::into),
			gas: r.gas.map(Into::into),
			value: r.value.map(Into::into),
			data: r.data.map(Into::into),
			nonce: r.nonce.map(Into::into),
		}
	}
}

impl From<helpers::FilledTransactionRequest> for TransactionRequest {
	fn from(r: helpers::FilledTransactionRequest) -> Self {
		TransactionRequest {
			from: Some(r.from),
			to: r.to,
			gas_price: Some(r.gas_price),
			gas: Some(r.gas),
			value: Some(r.value),
			data: Some(r.data.into()),
			nonce: r.nonce,

		}
	}
}

impl Into<helpers::TransactionRequest> for TransactionRequest {
	fn into(self) -> helpers::TransactionRequest {
		helpers::TransactionRequest {
			from: self.from.map(Into::into),
			to: self.to.map(Into::into),
			gas_price: self.gas_price.map(Into::into),
			gas: self.gas.map(Into::into),
			value: self.value.map(Into::into),
			data: self.data.map(Into::into),
			nonce: self.nonce.map(Into::into),

		}
	}
}