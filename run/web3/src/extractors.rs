use crate::http_common::HttpMetaExtractor;
use crate::metadata;
use crate::types;
use metadata::Metadata as MD;
use types::Origin;

/// Common HTTP.
pub struct RpcExtractor;
impl HttpMetaExtractor for RpcExtractor {
	type Metadata = MD;
	fn read_metadata(&self, origin: Option<String>, user_agent: Option<String>) -> MD {
		MD {
			origin: Origin::Rpc(format!(
				"{} / {}",
				origin.unwrap_or_else(|| "unknown origin".to_string()),
				user_agent.unwrap_or_else(|| "unknown agent".to_string())
			)),
		}
	}
}
