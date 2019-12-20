use ethereum_types::{Address, H256, U256, U512};
use state_provider::{StateProvider, Account as StateAccount};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Account {
    nonce: U256,
    balance: U256,
    code: Option<Vec<u8>>,
    changed: bool,
    storage: HashMap<U256, (Vec<u8>, bool)>,
}

impl Account {
    pub fn new(nonce: U256, balance: U256, code: Option<Vec<u8>>) -> Account {
        Account {
            nonce,
            balance,
            code,
            changed: false,
            storage: HashMap::new(),
        }
    }
}

pub struct Cache<'a> {
    provider: &'a mut StateProvider,
    accounts: HashMap<Address, Account>,
}

impl<'a> Cache<'a> {
    pub fn new(provider: &'a mut StateProvider) -> Self {
        Cache {
            provider: provider,
            accounts: HashMap::new(),
        }
    }

    ///
    pub fn create_account(
        &mut self,
        address: Address,
        nonce: U256,
        balance: U256,
        code: Option<Vec<u8>>,
    ) {
        // TODO
        let code1 = code.clone();
        let mut acc = Account::new(balance, nonce, code);
        acc.changed = true;
        /// TODO
        let _ret = self.accounts.insert(address, acc);

        self.provider.create_account(address, StateAccount{
            balance: balance,
            nonce: nonce,
            code: code1,
        }

        );
    }

    pub fn get_nonce(&mut self, address: &Address) -> Option<U256> {
        match self.get_account(address) {
            Some(acc) => Some(acc.nonce),
            _ => None,
        }
    }
    pub fn get_balance(&mut self, address: &Address) -> Option<U256> {
        match self.get_account(address) {
            Some(acc) => Some(acc.balance),
            _ => None,
        }
    }
    pub fn storage_at(&self, key: U256) -> U256 {
        U256::zero()
    }

    pub fn blockhash(&self, num: i64) -> U512 {
        U512::zero()
    }
    pub fn exist(&self, address: Address) -> bool {
        false
    }

    /*
    TODO
    fn get_account(&mut self, address: &Address) -> Option<&Account> {
        if let Some(acc) = self.accounts.get(address) {
            Some(acc)
        } else {
            if let Some(acc) = self.provider.get_account(*address) {
                let acc = Account::new(acc.balance, acc.nonce, acc.code);
                let _ret = self.accounts.insert(*address, acc);

                // TODO: check _ret
                None//self.get_account(address)
            } else {
                None
            }
        }
    }
    */

    fn get_account(&mut self, address: &Address) -> Option<Account> {
        if let Some(acc) = self.accounts.get(address) {
            Some(acc.clone())
        } else {
            if let Some(acc) = self.provider.get_account(*address) {
                let acc = Account::new(acc.balance, acc.nonce, acc.code);
                let _ret = self.accounts.insert(*address, acc);

                // TODO: check _ret
                self.get_account(address)
            } else {
                None
            }
        }
    }
}
