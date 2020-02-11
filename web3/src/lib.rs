extern crate blockchain;


pub mod rpc_apis;
pub mod rpc;
pub mod rpc_service;
pub mod http_common;


pub use jsonrpc_core::{Compatibility, Error, MetaIoHandler};
pub use jsonrpc_http_server::Server;
// pub use rpc_service::start_http;
extern crate log;
pub mod v1;
extern crate jsonrpc_http_server as http;
pub use http::{
	hyper,
	RequestMiddleware, RequestMiddlewareAction,
	AccessControlAllowOrigin, Host, DomainsValidation, cors::AccessControlAllowHeaders
};

use v1::Metadata;
use jsonrpc_core;
use jsonrpc_http_server::{self, ServerBuilder};
use std::io;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
extern crate jsonrpc_derive;
use http_common::HttpMetaExtractor;
pub use v1::extractors::RpcExtractor;

// pub use http::{
// 	cors::AccessControlAllowHeaders, hyper, AccessControlAllowOrigin, DomainsValidation, Host,
// 	RequestMiddleware, RequestMiddlewareAction,
// };

/// RPC HTTP Server instance
pub type HttpServer = http::Server;

/// Start http server asynchronously and returns result with `Server` handle on success or an error.
pub fn start_http<M, S, H, T>(
	addr: &SocketAddr,
	cors_domains: http::DomainsValidation<http::AccessControlAllowOrigin>,
	allowed_hosts: http::DomainsValidation<http::Host>,
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
	T: HttpMetaExtractor<Metadata=M>,
{
	let extractor = http_common::MetaExtractor::new(extractor);
	Ok(http::ServerBuilder::with_meta_extractor(handler, extractor)
		.keep_alive(keep_alive)
		.threads(threads)
		.cors(cors_domains)
		.allowed_hosts(allowed_hosts)
		.health_api(("/api/health", "durian"))
		.cors_allow_headers(AccessControlAllowHeaders::Any)
		.max_request_body_size(max_payload * 1024 * 1024)
		.start_http(addr)?)
}