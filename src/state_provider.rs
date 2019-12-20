use ethereum_types::{Address, U256, U512};

pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    pub code: Option<Vec<u8>>,
}

pub trait StateProvider {
    fn get_account(&self, address: Address) -> Option<Account>;
    fn storage_at(&self, key: U256) -> U256;
    fn blockhash(&self, num: i64) -> U512;
    fn exist(&self, address: Address) -> bool;

    fn create_account(&mut self, address: Address, account: Account);

}
