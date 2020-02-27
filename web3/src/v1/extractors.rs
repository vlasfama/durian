use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::fmt;
use crate::http_common::HttpMetaExtractor;
use jsonrpc_core as core;
use jsonrpc_core::futures::future::Either;
use ethereum_types::H256;
use jsonrpc_core;
use crate::v1;
use v1::Metadata as MD;
use v1::Origin;

/// Common HTTP.
pub struct RpcExtractor;
impl HttpMetaExtractor for RpcExtractor {
	type Metadata = MD;
	fn read_metadata(&self, origin: Option<String>, user_agent: Option<String>) -> MD {
		MD {
			origin: Origin::Rpc(
				format!("{} / {}",
						origin.unwrap_or_else(|| "unknown origin".to_string()),
						user_agent.unwrap_or_else(|| "unknown agent".to_string()))
			),

		}
	}
}


