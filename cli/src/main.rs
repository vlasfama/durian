#[macro_use]
use log::Level;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
mod rpc;


fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    rpc::initRPC();
}
