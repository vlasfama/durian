use crate::impls;
use crate::metadata;
use crate::traits;
use blockchain::blockchain::Blockchain;
use impls::TransactionRPCImpl;
pub use jsonrpc_core::{Compatibility, Error, MetaIoHandler};
use metadata::Metadata;
use std::collections::HashSet;
use traits::TransactionRPC;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Api {
	/// Transaction methods
	Transaction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ApiSet {
	List(HashSet<Api>),
}

impl Default for ApiSet {
	fn default() -> Self {
		ApiSet::List(vec![Api::Transaction].into_iter().collect())
	}
}

impl ApiSet {
	pub fn list_apis(&self) -> HashSet<Api> {
		match *self {
			ApiSet::List(ref apis) => apis.clone(),
		}
	}
}

pub fn setup_rpc(
	mut handler: MetaIoHandler<Metadata>,
	apis: ApiSet,
	bc: Blockchain,
) -> MetaIoHandler<Metadata> {
	for api in apis.list_apis() {
		match api {
			Api::Transaction => {
				handler.extend_with(TransactionRPCImpl::new(bc.clone()).to_delegate())
			}
		}
	}

	handler
}
