use ethereum_types::{Address, U512, U256};



pub trait StateProvider {
    fn storage_at(&self, key: U256) -> U256;
    fn blockhahs(&self, num: i64) -> U512;
    fn exist(&self, address: Address) -> bool;
}

