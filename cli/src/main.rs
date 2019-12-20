mod blockchain;

extern crate durian;

use blockchain::Blockchain;
use durian::stateless_vm::StatelessVM;
use durian::transaction::Transaction;
use ethereum_types::{Address, U256};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

fn main() {
    let mut bc = Blockchain::new();

    let file_path = "/home/mostafa/Downloads/pwasm_tutorial_contract.wasm";
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!(err.to_string()),
    };
    let mut code = Vec::new();
    if let Err(err) = file.read_to_end(&mut code) {
        panic!(err.to_string());
    }

    let data1 = vec![];
    let tx1 = Transaction::new(
        bc.get_address("alice"),
        Address::zero(),
        U256::from(10000),
        code,
        data1,
    );

    let vm = StatelessVM::new();

    let addr_1 = vm.fire(tx1, &mut bc);

    bc.commit();
}
