use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::{ServerBuilder};
use crate::blockchain::Blockchain;
use actix_web::{web, Result};
use serde::Deserialize;
use jsonrpc_core::types;
pub struct DeployRequest {
    pub code_hash: std::vec::Vec<u8>,
    pub address: std::vec::Vec<u8>,
    pub sender: std::vec::Vec<u8>,
    pub origin: std::vec::Vec<u8>,
    pub gas: u64,
    pub gas_price: u64,
    pub value: std::string::String,
    pub code: std::vec::Vec<u8>,
    pub data: std::vec::Vec<u8>,
}

pub struct contract_data {}

//call_contract
fn call_contract(params: Params){
    println!("json object contains {:?}",params);
   let bc =Blockchain::new();
   bc.call_contract(params)
}

pub fn start_rpc() {
	let mut io = IoHandler::new();
	io.add_method("call_contract", |_params: Params| {

		Ok(Value::String("hello".to_string()))
	});
	let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.unwrap();

	server.wait();
}