//web3 module

pub mod contract_call;
pub mod event_loop;
pub mod extractors;
pub mod helpers;
pub mod http_common;
pub mod impls;
pub mod metadata;
pub mod rpc;
pub mod rpc_apis;
pub mod rpc_service;
pub mod traits;
pub mod types;

pub use jsonrpc_core::{Compatibility, Error, MetaIoHandler};
pub use jsonrpc_http_server::Server;

// pub use rpc_service::start_http;
extern crate jsonrpc_http_server as http;
extern crate log;
pub use http::{
	cors::AccessControlAllowHeaders, hyper, AccessControlAllowOrigin, DomainsValidation, Host,
	RequestMiddleware, RequestMiddlewareAction,
};

use jsonrpc_core;
use jsonrpc_http_server::{self};
extern crate jsonrpc_derive;
pub use extractors::RpcExtractor;


fn main(){
  // Start web3 RPC Endpoint
    let mut el = event_loop::event_loop();
    let conf = rpc::HttpConfiguration::default();
    let server = rpc::new_http("HTTP JSON-RPC", "jsonrpc", conf);
    el.run(event_loop::forever()).unwrap();
}