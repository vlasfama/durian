use ethereum_types::{Address, H256, U256, U512};
use error::Error;

pub struct StateAccount {
    pub nonce: U256,
    pub balance: U256,
    pub code: Vec<u8>,
}

pub trait StateProvider {
    fn account(&self, address: &Address) -> Result<StateAccount, Error>;
    fn storage_at(&self, address: &Address, key: &H256) -> Result<H256, Error>;
    fn blockhash(&self, num: i64) -> U512;
    fn exist(&self, address: &Address) -> bool;

    fn create_contract(&mut self, address: Address, nonce: U256);
    fn init_code(&mut self, address: &Address, code: Vec<u8>);
    fn set_storage(&mut self, address: &Address, key: &H256, value: &H256);
}
