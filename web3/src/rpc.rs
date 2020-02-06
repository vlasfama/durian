use blockchain;
use blockchain::blockchain::Blockchain;
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params, Value};
// use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};
// use jsonrpc_core as rpc;
// use jsonrpc_http_server as http;

// //deploy the contract
fn call_contract(params: Params) {
	let mut bc = Blockchain::new();
	bc.call_contract(params);
}

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
///Hyper
use std::convert::Infallible;

async fn eth_gasPrice(params: Request<Body>) -> Result<Response<Body>, Infallible> {
	println!("the request body {:?}", params);
	// jsonvalue =
	Ok(Response::new(Body::from("{id:1,jsonrpc:2.0,result:0x0}")))
}

async fn eth_sendTransaction(params: Request<Body>) -> Result<Response<Body>, Infallible> {
	println!("the request body {:?}", params);
	Ok(Response::new(Body::from("Hello World!")))
}

#[tokio::main]
pub async fn start_rpc() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(eth_gasPrice)) });
	let addr = ([127, 0, 0, 1], 3030).into();
	let server = Server::bind(&addr).serve(make_svc);
	println!("Listening on http://{}", addr);
	server.await?;
	Ok(())
}
