#[derive(Serialize, Deserialize)]
use serde::{Serialize, Deserialize};
use ethereum_types::{Address};
use rocket::State;
use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;

pub struct contract {
    pub from: Address,
    pub to:  Address,
    pub data: String,
    pub value: String,
    pub gas: u64,
    pub gasPrice: u64,
    pub privKey: String,
    pub nonce: u32

}

#[get("/world")]
fn world()  {
  let msg = "hello world";
  println!("the message is {}", msg);
}

#[post("/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/call")]
fn call(state: State<contract>) -> String {
    format!("The config value is: {}","hel")
}

//deploy the contract
#[post("/deploy")]
fn deploy(state: State<contract>) -> String {
    format!("The config value is: {}","hel")
}

pub fn start_RestApi() {
    rocket::ignite()
        .mount("/hello", routes![hello])
        .mount("/world", routes![world])
        .launch();
}