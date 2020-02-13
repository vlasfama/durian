use crate::rpc_apis::{self, Api, ApiSet};
use crate::rpc_service::{self as rpc, start_http};
use crate::v1::{extractors,Metadata};
use extractors::{RpcExtractor};
use jsonrpc_core::{Compatibility, MetaIoHandler};
pub use jsonrpc_http_server::{DomainsValidation, Server};
use std::collections::HashSet;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;


/// RPC HTTP Server instance
pub type HttpServer = http::Server;
pub const DAPPS_DOMAIN: &'static str = "web3.site";

#[derive(Debug, Clone, PartialEq)]
pub struct HttpConfiguration {
	/// Is RPC over HTTP enabled (default is true)?
	pub enabled: bool,
	/// The IP of the network interface used (default is 127.0.0.1).
	pub interface: String,
	/// The network port (default is 8545).
	pub port: u16,
	/// The categories of RPC calls enabled.
	pub apis: ApiSet,
	/// CORS headers
	pub cors: Option<Vec<String>>,
	/// Specify a list of valid hosts we accept requests from.
	pub hosts: Option<Vec<String>>,
	/// Number of HTTP server threads to use to handle incoming requests (default is 4).
	pub server_threads: usize,
	/// Sets the maximum size of a request body in megabytes (default is 5 MiB).
	pub max_payload: usize,
	/// Use keepalive messages on the underlying socket: SO_KEEPALIVE as well as the TCP_KEEPALIVE
	/// or TCP_KEEPIDLE options depending on your platform (default is true).
	pub keep_alive: bool,
}

impl Default for HttpConfiguration {
	fn default() -> Self {
		HttpConfiguration {
			enabled: true,
			interface: "127.0.0.1".into(),
			port: 8545,
			apis: ApiSet::default(),
			cors: Some(vec![]),
			hosts: Some(vec![]),
			server_threads: 4,
			max_payload: 5,
			keep_alive: true,
		}
	}
}

pub fn new_http(
	id: &str,
	options: &str,
	conf: HttpConfiguration,
) -> Result<Option<HttpServer>, String> {
	if !conf.enabled {
		return Ok(None);
	}
	let domain = DAPPS_DOMAIN;
	let url = format!("{}:{}", conf.interface, conf.port);
	let addr = url
		.parse()
		.map_err(|_| format!("Invalid {} listen host/port given: {}", id, url))?;
	let handler = rpc_apis::setup_rpc(
		MetaIoHandler::with_compatibility(Compatibility::Both),
		conf.apis,
	);

	let cors_domains = into_domains(conf.cors);
	let allowed_hosts = into_domains(with_domain(conf.hosts, domain, &Some(url.clone().into())));

	let start_result = start_http(&addr,cors_domains,
		allowed_hosts,
		handler,
		RpcExtractor,
		conf.server_threads,
		conf.max_payload,
		conf.keep_alive,
	);
	match start_result {
		Ok(server) => Ok(Some(server)),
		Err(ref err) if err.kind() == io::ErrorKind::AddrInUse => Err(
			format!("{} address {} is already in use, make sure that another instance of an Ethereum client is not running or change the address using the --{}-port and --{}-interface options.", id, url, options, options)
		),
		Err(e) => Err(format!("{} error: {:?}", id, e)),
	}
}

fn setup_rpc_server(apis: ApiSet) -> MetaIoHandler<Metadata> {
	rpc_apis::setup_rpc(MetaIoHandler::with_compatibility(Compatibility::Both), apis)
}

fn into_domains<T: From<String>>(items: Option<Vec<String>>) -> DomainsValidation<T> {
	items
		.map(|vals| vals.into_iter().map(T::from).collect())
		.into()
}

fn with_domain(
	items: Option<Vec<String>>,
	domain: &str,
	dapps_address: &Option<rpc::Host>,
) -> Option<Vec<String>> {
	fn extract_port(s: &str) -> Option<u16> {
		s.split(':').nth(1).and_then(|s| s.parse().ok())
	}

	items.map(move |items| {
		let mut items = items.into_iter().collect::<HashSet<_>>();
		{
			let mut add_hosts = |address: &Option<rpc::Host>| {
				if let Some(host) = address.clone() {
					items.insert(host.to_string());
					items.insert(host.replace("127.0.0.1", "localhost"));
					items.insert(format!("http://*.{}", domain)); //proxypac
					if let Some(port) = extract_port(&*host) {
						items.insert(format!("http://*.{}:{}", domain, port));
					}
				}
			};

			add_hosts(dapps_address);
		}
		items.into_iter().collect()
	})
}

