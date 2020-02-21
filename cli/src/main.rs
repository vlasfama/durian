#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate hex_literal;
extern crate simple_logger;
extern crate web3;

use log::Level;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use web3::rpc;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let mut conf = rpc::HttpConfiguration::default();
    rpc::new_http("HTTP JSON-RPC", "jsonrpc", conf);
}
