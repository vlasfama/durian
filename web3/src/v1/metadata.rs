//! Durian RPC requests Metadata.
use std::sync::Arc;
use crate::v1;
use jsonrpc_core;
use v1::types::Origin;


/// RPC methods metadata.
#[derive(Clone, Default, Debug)]
pub struct Metadata {
	/// Request origin
	pub origin: Origin
}

impl jsonrpc_core::Metadata for Metadata {}

