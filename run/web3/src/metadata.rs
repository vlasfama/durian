//! Durian RPC requests Metadata.
use crate::types;
use jsonrpc_core;
use types::Origin;


/// RPC methods metadata.
#[derive(Clone, Default, Debug)]
pub struct Metadata {
	/// Request origin
	pub origin: Origin
}
impl jsonrpc_core::Metadata for Metadata {}

