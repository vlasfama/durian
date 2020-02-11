use ethereum_types::{Address, H256, U256, U512};
use state_provider::StateProvider;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct AccountInfo {
    nonce: U256,
    balance: U256,
    code: Vec<u8>,
    storage: HashMap<H256, (H256, bool)>,
}

impl AccountInfo {
    pub fn new(nonce: U256, balance: U256, code: Vec<u8>) -> AccountInfo {
        AccountInfo {
            nonce,
            balance,
            code,
            storage: HashMap::new(),
        }
    }
}

pub struct StateCache<'a, T: StateProvider> {
    provider: &'a mut T,
    accounts: HashMap<Address, (AccountInfo, bool)>,
}

impl<'a, T> StateCache<'a, T>
where
    T: StateProvider,
{
    pub fn new(provider: &'a mut T) -> Self {
        StateCache {
            provider: provider,
            accounts: HashMap::new(),
        }
    }

    ///
    pub fn create_contract(&mut self, address: Address, nonce: U256) {
        let acc = AccountInfo::new(U256::zero(), nonce, vec![]);
        let ret = self.accounts.insert(address, (acc, true));

        assert_eq!(ret, None);
    }

    pub fn nonce(&mut self, address: &Address) -> vm::Result<U256> {
        let acc = self.account(address)?;
        Ok(acc.nonce)
    }

    pub fn balance(&mut self, address: &Address) -> vm::Result<U256> {
        let acc = self.account(address)?;
        Ok(acc.balance)
    }

    pub fn storage_at(&mut self, address: &Address, key: &H256) -> vm::Result<H256> {
        self.fetch_storage(address, key)?;

        let acc = self.account(address)?;
        return Ok(acc.storage.get(key).unwrap().0);
    }

    pub fn set_storage(&mut self, address: &Address, key: &H256, value: &H256) {
        let acc = self.account_mut(address).unwrap();
        acc.0.storage.insert(*key, (*value, true));
    }

    pub fn blockhash(&self, _num: i64) -> U512 {
        U512::zero()
    }

    pub fn exist(&self, _address: Address) -> bool {
        false
    }

    fn account_mut(&mut self, address: &Address) -> vm::Result<&mut (AccountInfo, bool)> {
        self.fetch_account(address)?;

        return Ok(self.accounts.get_mut(address).unwrap());
    }

    fn account(&mut self, address: &Address) -> vm::Result<&AccountInfo> {
        self.fetch_account(address)?;

        return Ok(&self.accounts.get(address).unwrap().0);
    }

    pub fn init_code(&mut self, address: &Address, code: Vec<u8>) {
        let mut acc = self.account_mut(address).unwrap();
        acc.0.code = code;
        acc.1 = true;
    }

    fn fetch_account(&mut self, address: &Address) -> vm::Result<()> {
        if self.accounts.contains_key(address) {
            return Ok(());
        }

        if let Ok(acc) = self.provider.account(address) {
            let acc = AccountInfo::new(acc.balance, acc.nonce, acc.code);
            self.accounts.insert(*address, (acc, false));
            Ok(())
        } else {
            Err(vm::Error::Internal(format!(
                "Invalid address: {:?}",
                address
            )))
        }
    }

    fn fetch_storage(&mut self, address: &Address, key: &H256) -> vm::Result<()> {
        let acc = self.account(address)?;
        if acc.storage.contains_key(key) {
            return Ok(());
        }

        if let Ok(value) = self.provider.storage_at(address, key) {
            let acc = self.account_mut(address)?;
            acc.0.storage.insert(*key, (value, false));
            Ok(())
        } else {
            Err(vm::Error::Internal(format!("Invalid storage: {:?}", key)))
        }
    }
}
