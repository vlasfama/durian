
use crate::v1;
use blockchain::blockchain::Blockchain;
use ethereum_types::{H160,U256};
use jsonrpc_core::{Result};
use std::sync::{Mutex};
use v1::metadata::Metadata;
use v1::traits::AccountRPC;
use v1::types::CallRequest;
use v1::types::{BlockNumber};

pub struct AccountRPCImpl {
    bc: Mutex<Blockchain>,
}

impl AccountRPCImpl {
    pub fn new(bc: Blockchain) -> Self {
        AccountRPCImpl { bc: Mutex::new(bc) }
    }
}

impl AccountRPC for AccountRPCImpl {
    type Metadata = Metadata;

    //Get the account balance
    fn balance(&self, address: H160, num: Option<BlockNumber>) -> Result<U256> {
        let num = num.unwrap_or_default();
        let mut bc = self.bc.lock().unwrap();
        let res = bc.get_balance(address);
        Ok(res)
    }
    //Get the account balance
    fn estimate_gas(&self, request: CallRequest, num: Option<BlockNumber>) -> Result<U256> {
        Ok(U256::zero())
    }


}
