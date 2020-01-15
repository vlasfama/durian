use std::collections::BTreeMap;
use ethereum_types::{H64, H160, H256, H512, U64, U256};
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;
use v1::types::{
	Bytes, CallRequest,
	Peers, Transaction, RpcSettings, Histogram, RecoveredAccount,
	TransactionStats, LocalTransactionStatus,
	BlockNumber, ConsensusCapability, VersionInfo,
	OperationsInfo, ChainStatus, Log, Filter,
	RichHeader, Receipt,
};

/// Parity-specific rpc interface.
#[rpc(server)]
pub trait Parity {
	/// RPC Metadata
	type Metadata;
	/// Call contract, returning the output data.
	#[rpc(name = "parity_call")]
	fn call(&self, _: Vec<CallRequest>, _: Option<BlockNumber>) -> Result<Vec<Bytes>>;

	/// Used for submitting a proof-of-work solution (similar to `eth_submitWork`,
	/// but returns block hash on success, and returns an explicit error message on failure).
}
