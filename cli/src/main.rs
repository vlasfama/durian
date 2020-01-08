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

    let file_path = "./compiled-contract/pwasm_erc20_token.wasm";
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!(err.to_string()),
    };
    let mut code = Vec::new();
    if let Err(err) = file.read_to_end(&mut code) {
        panic!(err.to_string());
    }

    //let total = U256::from(1000);
    let data1 = vec![1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2];
    //let data1 = vec![1,2,3,4,5,6,7,8,9,0,1,2,3];
    let tx1 = Transaction::new(
        bc.address("alice"),
        Address::zero(),
        U256::from(10000000),
        code,
        data1,
    );

    let vm = StatelessVM::new();

    let addr_1 = vm.fire(tx1, &mut bc);
    let code = bc.code("contract");

    let data1 = vec![];
    let tx1 = Transaction::new(
        bc.address("alice"),
        addr_1,
        U256::from(10000),
        code,
        data1,
    );

    let addr_1 = vm.fire(tx1, &mut bc);

    bc.commit();
}
