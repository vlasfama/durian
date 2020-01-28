
#![feature(proc_macro_hygiene, decl_macro)]
use serde::{Serialize, Deserialize};
mod blockchain;
extern crate durian;
// extern crate rpc_durian;
#[macro_use]
extern crate log;
extern crate simple_logger;
use blockchain::Blockchain;
use durian::stateless_vm::StatelessVM;
use durian::transaction::{Action, Transaction};
use ethereum_types::{Address, U256};
use log::Level;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
// use rpc_durian::{service};
mod rpc;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let mut bc = Blockchain::new();
    let file_path = "./compiled-contract/pwasm_greeter.wasm";
    // rpc::start_RestApi();

    // rpc_durian::service::rpc_server::start_rpc();
    rpc::start_rpc();

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!(err.to_string()),
    };

    let mut code = Vec::new();
    if let Err(err) = file.read_to_end(&mut code) {
        panic!(err.to_string());
    }

    let tx1 = Transaction::create(
        bc.address("alice"),
        U256::zero(),
        U256::from(10000000),
        code,
        vec![],
    );

    let vm = StatelessVM::new();

    let ret_1 = vm.fire(tx1, &mut bc).unwrap();
    bc.incNonce("alice");
    bc.commit();

    let addr_1 = ret_1.contract;
    let params = vec![
        0x40, 0x18, 0xd9, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0x12, 0x34,
    ];
    let tx2 = Transaction::call(
        bc.address("alice"),
        addr_1,
        U256::zero(),
        U256::from(1000000),
        vec![],
        params,
    );
    let ret_2 = vm.fire(tx2, &mut bc);
    bc.incNonce("alice");
    bc.commit();

    let params = vec![0x51, 0x97, 0xc7, 0xaa];
    let tx3 = Transaction::call(
        bc.address("alice"),
        addr_1,
        U256::zero(),
        U256::from(1000000),
        vec![],
        params,
    );
    let ret_3 = vm.fire(tx3, &mut bc);
    bc.incNonce("alice");

    bc.commit();



}


