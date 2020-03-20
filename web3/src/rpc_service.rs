use jsonrpc_core;
use std::net::SocketAddr;
extern crate jsonrpc_derive;
extern crate jsonrpc_http_server as http;
use crate::extractors;
use crate::http_common;
use http_common::HttpMetaExtractor;

pub use extractors::RpcExtractor;

pub use http::{
	cors::AccessControlAllowHeaders, hyper, AccessControlAllowOrigin, DomainsValidation, Host,
	RequestMiddleware, RequestMiddlewareAction,
};

/// RPC HTTP Server instance
pub type HttpServer = http::Server;

/// Start http server asynchronously and returns result with `Server` handle on success or an error.
pub fn start_http<M, S, H, T>(
	addr: &SocketAddr,
	cors_domains: http::DomainsValidation<http::AccessControlAllowOrigin>,
	handler: H,
	extractor: T,
	threads: usize,
	max_payload: usize,
	keep_alive: bool,
) -> ::std::io::Result<HttpServer>
where
	M: jsonrpc_core::Metadata,
	S: jsonrpc_core::Middleware<M>,
	H: Into<jsonrpc_core::MetaIoHandler<M, S>>,
	T: HttpMetaExtractor<Metadata = M>,
{
	let extractor = http_common::MetaExtractor::new(extractor);
	Ok(http::ServerBuilder::with_meta_extractor(handler, extractor)
		.keep_alive(keep_alive)
		.threads(threads)
		.cors(cors_domains)
		.health_api(("/api/health", "durian_value"))
		.cors_allow_headers(AccessControlAllowHeaders::Any)
		.max_request_body_size(max_payload * 1024 * 1024)
		.start_http(addr)?)
}
