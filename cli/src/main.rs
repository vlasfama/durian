#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate hex_literal;
extern crate rpchyper;
extern crate simple_logger;
extern crate web3;

use log::Level;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use web3::rpc;

fn main() {
    // let mut server = event_loop::event_loop();
    simple_logger::init_with_level(Level::Info).unwrap();
    rpchyper::rpc_hyper::Start();
    // server.run(event_loop::forever()).unwrap();
}
