mod blockchain;

extern crate durian;

use ethereum_types::{U256, Address};
use blockchain::Blockchain;
use durian::stateless_vm::StatelessVM;
use durian::transaction::Transaction;
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

   // let mut data = vec![1,2,3];


    //
    let tx = Transaction::new(
        bc.get_address("alice"),
        bc.get_address("bob"),
        //Address::zero(),
        U256::from(10000),
        Some(Arc::new(code)),
        //Some(data),
        None,
    );

    let vm = StatelessVM::new();

    let res = match vm.fire(tx, &bc) {
        Ok(res) => res,
        Err(err) => panic!("error"),
    };


    bc.commit();


}
