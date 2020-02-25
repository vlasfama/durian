extern crate jsonrpc_http_server;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};

pub fn initRPC() {
	let mut io = IoHandler::default();
	io.add_method("eth_gasPrice", |_params| {
		futures::finished(Value::String("0".to_owned()))
	});

	io.add_method("eth_sendTransaction", |_params| {
		println!("{:?}", _params);
		futures::finished(Value::String("0".to_owned()))
	});

	let server = ServerBuilder::new(io)
		.cors(DomainsValidation::AllowOnly(vec![
			AccessControlAllowOrigin::Null,
		]))
		.start_http(&"127.0.0.1:8545".parse().unwrap())
		.expect("Unable to start RPC server");

	server.wait();
}
