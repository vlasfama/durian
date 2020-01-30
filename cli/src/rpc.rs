
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::{ServerBuilder,AccessControlAllowOrigin, DomainsValidation, RestApi,};
use crate::blockchain::Blockchain;



//deploy the contract
fn call_contract(params: Params){
    let mut bc = Blockchain::new();
    bc.call_contract(params);
	// println!("enter the cal_contract {:?}",params)
}

//init the rpc
pub fn start_rpc(){
	let mut io = IoHandler::default();
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