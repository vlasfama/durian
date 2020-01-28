use crate::jsonrpc_core::*;
use crate::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};
// use crate::{blockchain};

//deploy the contract
fn call_contract(params: Params){
	println!("enter the cal_contract {:?}",params)
}

//iniit the rpc
pub fn main(){
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params: Params| Ok(Value::String("hello".to_string())));
    io.add_method("eth_sendTransaction", |_params: Params| {
		call_contract(_params);
		Ok(Value::String("hello".into()))
	});
	let server = ServerBuilder::new(io)
		.threads(3)
		.rest_api(RestApi::Unsecure)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.expect("Unable to start RPC server");

	server.wait();
}