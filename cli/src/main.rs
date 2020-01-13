mod blockchain;

extern crate durian;
#[macro_use]
extern crate log;
extern crate simple_logger;

use blockchain::Blockchain;
use durian::stateless_vm::StatelessVM;
use durian::transaction::{Transaction, Action};
use ethereum_types::{Address, U256};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;


fn main() {
    simple_logger::init().unwrap();
    warn!("This is an example message.");

    let mut bc = Blockchain::new();

    let file_path = "./compiled-contract/pwasm_greeter.wasm";
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
        U256::from(10000000),
        U256::zero(),
        code,
        vec![],
    );

    let vm = StatelessVM::new();

    let code = vm.fire(tx1, &mut bc);
    let addr_1 = bc.address("contract_0");
    let code = bc.code("contract_0");
    
    let params = vec![0x40,0x18,0xd9,0xaa,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x12,0x34];

    let tx2 = Transaction::call(
        bc.address("alice"),
        addr_1,
        U256::from(1000000),
        U256::zero(),
        code,
        params,
    );

    let addr_2 = vm.fire(tx2, &mut bc);

    bc.commit();



}
