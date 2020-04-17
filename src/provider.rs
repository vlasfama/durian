use error::Result;
use ethereum_types::{Address, H256, U256};

pub struct StateAccount {
    pub nonce: U256,
    pub balance: U256,
    pub code: Vec<u8>,
}

pub trait Provider {
    fn exist(&self, address: &Address) -> bool;
    fn account(&self, address: &Address) -> Result<StateAccount>;
    fn update_account(&mut self, address: &Address, bal: &U256, nonce: &U256) -> Result<()>;
    fn create_contract(&mut self, address: &Address, code: &Vec<u8>) -> Result<()>;

    fn storage_at(&self, address: &Address, key: &H256) -> Result<H256>;
    fn set_storage(&mut self, address: &Address, key: &H256, value: &H256) -> Result<()>;

    fn timestamp(&self) -> u64;
    fn block_number(&self) -> u64;
    fn block_hash(&self, block_no: u64) -> Result<H256>;
    fn block_author(&self) -> Result<Address>;
    fn difficulty(&self) -> Result<U256>;
    fn gas_limit(&self) -> Result<U256>;
}
