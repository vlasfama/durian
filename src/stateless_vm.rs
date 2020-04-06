use error::{Error, Result};
use ethereum_types::{Address, H256, U256};
use interpreter::Interpreter;
use keccak_hash::write_keccak;
use log_entry::LogEntry;
use schedule::Schedule;
use serde::{Deserialize, Serialize};
use state_provider::StateProvider;
use std::collections::BTreeMap;
use std::sync::Arc;
use transaction::{Action, Transaction};
use types::{ActionParams, ActionType, ActionValue, GasLeft, ParamsType};
use utils;
use wasm_cost::WasmCosts;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResultData {
    pub gas_left: U256,
    pub data: Vec<u8>,
    pub apply_state: bool,
    pub contract: Address,
    pub logs: Vec<LogEntry>,
}

pub struct StatelessVM {}

impl StatelessVM {
    pub fn new() -> Self {
        StatelessVM {}
    }

    pub fn fire<T: StateProvider>(
        &self,
        transaction: &Transaction,
        provider: &mut T,
    ) -> Result<ResultData> {
        let params = match transaction.action {
            Action::Create => {
                let acc = provider.account(&transaction.sender)?;
                let new_address = utils::contract_address(
                    &transaction.sender,
                    &acc.nonce,
                    &transaction.code,
                    &H256::zero(), // TODO: SALT should be passed through the transaction????
                );

                ActionParams {
                    code_address: new_address.clone(),
                    address: new_address.clone(),
                    sender: transaction.sender.clone(),
                    origin: transaction.sender.clone(),
                    gas: transaction.gas,
                    value: ActionValue::Transfer(transaction.value),
                    code: Some(Arc::new(transaction.code)),
                    data: Some(transaction.params),
                    action_type: ActionType::Create,
                    params_type: ParamsType::Embedded,
                    gas_price: U256::zero(),
                    code_hash: None,
                    code_version: U256::zero(),
                }
            }
            Action::Call(ref address) => {
                let mut code = transaction.code;

                if code.is_empty() {
                    let acc = provider.account(&address)?;
                    code = acc.code.clone();
                }

                ActionParams {
                    code_address: address.clone(),
                    address: address.clone(),
                    sender: transaction.sender.clone(),
                    origin: transaction.sender.clone(),
                    gas: transaction.gas,
                    value: ActionValue::Transfer(transaction.value),
                    code: Some(Arc::new(code)),
                    data: Some(transaction.params),
                    action_type: ActionType::Call,
                    params_type: ParamsType::Separate,
                    gas_price: U256::zero(),
                    code_hash: None,
                    code_version: U256::zero(),
                }
            }
        };

        let mut schedule = Schedule::default();
        let wasm = WasmCosts::default();
        schedule.wasm = Some(wasm);

        let interpreter = Interpreter::new(params.clone());

        let ret = interpreter.run(provider, &schedule)?;

        match ret {
            GasLeft::Known(gas_left) => Ok(ResultData {
                gas_left,
                apply_state: true,
                data: vec![],
                contract: params.address,
                // TODO::::: logs????
                logs: vec![],// ext.logs().to_vec(),
            }),
            GasLeft::NeedsReturn {
                gas_left,
                data,
                apply_state,
            } => {
                // TODO: Can we move it to runtime?
                //if transaction.action == Action::Create {
                //    provider.create_contract(&params.address, 0, data.to_vec());
                //}

                //ext.update_state()?;

                Ok(ResultData {
                    gas_left,
                    apply_state: apply_state,
                    data: data.to_vec(),
                    contract: params.address,
                    logs: ext.logs().to_vec(),
                })
            }
        }
    }
}
