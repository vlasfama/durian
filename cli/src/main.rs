#[macro_use]
// extern crate web3;
use log::Level;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
// use web3::rpc;
mod event_loop;
mod rpc;


fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    // let mut el = event_loop::event_loop();
    // let conf = rpc::HttpConfiguration::default();
    // let server = rpc::new_http("HTTP JSON-RPC", "jsonrpc", conf);
    // el.run(event_loop::forever()).unwrap();
    rpc::InitRPC();
}
