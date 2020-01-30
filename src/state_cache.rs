use ethereum_types::{Address, H256, U256, U512};
use state_provider::{StateAccount, StateProvider};
use std::{
    cell::{RefCell, RefMut},
    collections::hash_map::Entry,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt,
    sync::Arc,
};

#[derive(Debug, Clone)]
struct AccountInfo {
    nonce: U256,
    balance: U256,
    code: Vec<u8>,
    changed: bool,
    storage: HashMap<H256, H256>,
}

impl AccountInfo {
    pub fn new(nonce: U256, balance: U256, code: Vec<u8>) -> AccountInfo {
        AccountInfo {
            nonce,
            balance,
            code,
            changed: false,
            storage: HashMap::new(),
        }
    }
}

pub struct StateCache<'a> {
    provider: &'a mut StateProvider,
    accounts: RefCell<HashMap<Address, AccountInfo>>,
}

impl<'a> StateCache<'a> {
    pub fn new(provider: &'a mut dyn StateProvider) -> Self {
        StateCache {
            provider: provider,
            accounts: RefCell::new(HashMap::new()),
        }
    }

    ///
    pub fn create_contract(&mut self, address: Address, nonce: U256) {
        // TODO
        let mut acc = AccountInfo::new(U256::zero(), nonce, vec![]);
        acc.changed = true;
        /// TODO
        let _ret = self.accounts.get_mut().insert(address, acc);

        self.provider.create_contract(address, nonce);
    }

    pub fn nonce(&mut self, address: &Address) -> vm::Result<U256> {
        let acc = self.account(address)?;
        Ok(acc.nonce)
    }

    pub fn balance(&mut self, address: &Address) -> vm::Result<U256> {
        let acc = self.account(address)?;
        Ok(acc.balance)
    }

    pub fn storage_at(&self, address: &Address, key: &H256) -> vm::Result<H256> {
        let acc = self.account(address)?;
        if let Some(storage) = acc.storage.get(key) {
            return Ok(*storage);
        } else {
            //// TODO
            /// let storage = self.provider.storage_at(address, key)?;
            let storage = self
                .provider
                .storage_at(address, key)
                .unwrap_or(H256::zero());
            return Ok(storage);
        }
    }

    pub fn set_storage(&mut self, address: &Address, key: &H256, value: &H256) {
        let mut acc = self.account(address).unwrap();
        let val = acc.storage.entry(*key).or_insert(*value);
        *val = *value;

        /// TEMP
        self.provider.set_storage(address, key, value);
    }

    pub fn blockhash(&self, num: i64) -> U512 {
        U512::zero()
    }

    pub fn exist(&self, address: Address) -> bool {
        false
    }

    /*
    TODO
    fn account(&mut self, address: &Address) -> Option<&Account> {
        if let Some(acc) = self.accounts.get(address) {
            Some(acc)
        } else {
            if let Some(acc) = self.provider.account(*address) {
                let acc = Account::new(acc.balance, acc.nonce, acc.code);
                let _ret = self.accounts.insert(*address, acc);

                // TODO: check _ret
                None//self.account(address)
            } else {
                None
            }
        }
    }
    */

    fn account(&self, address: &Address) -> vm::Result<AccountInfo> {
        let mut accs = self.accounts.borrow_mut();
        if let Some(acc) = accs.get(address) {
            Ok(acc.clone())
        } else {
            if let Ok(acc) = self.provider.account(address) {
                let acc = AccountInfo::new(acc.balance, acc.nonce, acc.code);
                let acc2 = acc.clone();

                accs.insert(*address, acc);
                Ok(acc2)

            // TODO:
            //self.account(address)
            } else {
                Err(vm::Error::Internal("Invalid address".to_string()))
            }
        }
    }

    pub fn init_code(&mut self, address: &Address, code: Vec<u8>) {
        /// TEMP
        self.provider.init_code(address, code);
    }
}
