use error::Error;
use ethereum_types::{Address, H256, U256, U512};

pub struct StateAccount {
    pub nonce: U256,
    pub balance: U256,
    pub code: Vec<u8>,
}

pub trait Provider {
    fn account(&self, address: &Address) -> Result<StateAccount, Error>;
    fn storage_at(&self, address: &Address, key: &H256) -> Result<H256, Error>;
    fn blockhash(&self, num: i64) -> H256;
    fn exist(&self, address: &Address) -> bool;
    fn create_contract(
        &mut self,
        address: &Address,
        nonce: &U256,
        code: &Vec<u8>,
    ) -> Result<(), Error>;
    fn set_storage(&mut self, address: &Address, key: &H256, value: &H256) -> Result<(), Error>;
    fn update_account(&mut self, address: &Address, bal: &U256, nonce: &U256) -> Result<(), Error>;
}
