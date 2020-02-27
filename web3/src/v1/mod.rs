
// short for "try_boxfuture"
// unwrap a result, returning a BoxFuture<_, Err> on failure.
macro_rules! try_bf {
	($res: expr) => {
		match $res {
			Ok(val) => val,
			Err(e) => return Box::new(::jsonrpc_core::futures::future::err(e.into())),
		}
	}
}

#[macro_use]
pub mod helpers;
pub mod impls;
pub mod traits;
pub mod types;
pub mod extractors;
pub mod metadata;




pub use self::traits::TransactionRPC;
pub use self::extractors::{RpcExtractor};
pub use self::helpers::{TransactionRequest,FilledTransactionRequest};
pub use self::metadata::Metadata;
pub use self::types::Origin;
