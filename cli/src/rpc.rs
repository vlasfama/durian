// use jsonrpc_http_server::jsonrpc_core::*;
// use jsonrpc_http_server::*;

// pub fn InitRPC() {
// 	let mut io = IoHandler::default();
// 	io.add_method("gas_Price", |_| {
// 		println!("hhhhh");
// 		Ok(Value::String("Hello World!".into()))
// 	});
// 	io.add_method("eth_Transaction", |_| {
// 		println!("hhhhh");
// 		Ok(Value::String("Hello World!".into()))
// 	});

// 	let server = ServerBuilder::new(io)
// 		.cors(DomainsValidation::AllowOnly(vec![
// 			AccessControlAllowOrigin::Null,
// 		]))
// 		.start_http(&"127.0.0.1:3030".parse().unwrap())
// 		.expect("Unable to start RPC server");

// 	server.wait();
// }

#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;

use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use hyper::{Get, Post, StatusCode};
use hyper::server::Http

static INDEX: &'static [u8] = b"Try POST /echo";

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
	type Request = Request;
	type Response = Response;
	type Error = hyper::Error;
	type Future = ::futures::Finished<Response, hyper::Error>;

	fn call(&self, req: Request) -> Self::Future {
		::futures::finished(match (req.method(), req.path()) {
			(&Get, "/") | (&Get, "/echo") => Response::new()
				.with_header(ContentLength(INDEX.len() as u64))
				.with_body(INDEX),
			(&Post, "/echo") => {
				let mut res = Response::new();
				if let Some(len) = req.headers().get::<ContentLength>() {
					res.headers_mut().set(len.clone());
				}
				res.with_body(req.body())
			}
			(&Post, "/get_gasPrice") => {
				let mut res = Response::new();
				if let Some(len) = req.headers().get::<ContentLength>() {
					res.headers_mut().set(len.clone());
				}
				res.with_body(req.body())
			}
			(&Post, "/eth_sendTransaction") => {
				let mut res = Response::new();
				if let Some(len) = req.headers().get::<ContentLength>() {
					res.headers_mut().set(len.clone());
				}
				res.with_body(req.body())
			}
			_ => Response::new().with_status(StatusCode::NotFound),
		})
	}
}

pub fn InitRPC() {
	pretty_env_logger::init().unwrap();
	let addr = "127.0.0.1:1337".parse().unwrap();

	let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
	println!(
		"Listening on http://{} with 1 thread.",
		server.local_addr().unwrap()
	);
	server.run().unwrap();
}
