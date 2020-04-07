extern crate durian;
extern crate ethereum_types;
extern crate simple_logger;
extern crate blockchain;
#[macro_use]
extern crate log;

use blockchain::blockchain::Blockchain;
use durian::execute;
use durian::transaction::Transaction;
use ethereum_types::{H256, U256};
use log::Level;
use std::fs::File;
use std::io::Read;

fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();

    let mut bc = Blockchain::new();

    // TODO:
    // please don't add lib.rs. It is command line. use main.rs (this file)
    // Move everything from web3 folder.
    // Try to avoid unnecessary crates
    // Look at cli to check the changes on durian



}
