use common_types::engines::params::CommonParams;
use error::Error;
use ethereum_types::{Address, U256};
use machine::Machine;
use state_provider::StateProvider;
use stateless_ext::{StatelessExt, LogEntry};
use std::collections::BTreeMap;
use std::sync::Arc;
use transaction::{Action, Transaction};
use vm::{ActionParams, ActionValue, CallType, EnvInfo, Exec, GasLeft, ParamsType};
use wasm::WasmInterpreter;




#[derive(Debug)]
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
        transaction: Transaction,
        provider: &mut T,
    ) -> ::std::result::Result<ResultData, Error> {
        let params = match transaction.action {
            Action::Create => {
                let acc = provider.account(&transaction.sender)?;
                let (new_address, _) = machine::executive::contract_address(
                    vm::CreateContractAddress::FromSenderAndNonce,
                    &transaction.sender,
                    &acc.nonce,
                    &vec![],
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
                    call_type: CallType::None,
                    params_type: ParamsType::Separate,
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
                    call_type: CallType::Call,
                    params_type: ParamsType::Separate,
                    gas_price: U256::zero(),
                    code_hash: None,
                    code_version: U256::zero(),
                }
            }
        };

        let mut env_info = EnvInfo::default();
        // TODO:::::
        env_info.timestamp = 111;
        env_info.gas_limit = U256::from(100000000);
        env_info.number = 1;

        let machine_params = CommonParams::default();
        let builtins = BTreeMap::default();
        let machine = Machine::regular(machine_params, builtins);
        let mut schedule = machine.schedule(env_info.number);
        let depth = 0;

        let wasm = vm::WasmCosts::default();
        schedule.wasm = Some(wasm);

        let mut ext = StatelessExt::new(&env_info, &machine, &schedule, depth, &params, provider);

        let interpreter = Box::new(WasmInterpreter::new(params.clone()));

        let ret = interpreter.exec(&mut ext);
        match ret {
            Ok(val) => match val {
                Ok(GasLeft::Known(gas_left)) => Ok(ResultData {
                    gas_left,
                    apply_state: true,
                    data: vec![],
                    contract: params.address,
                    logs: ext.logs().to_vec(),
                }),
                Ok(GasLeft::NeedsReturn {
                    gas_left,
                    data,
                    apply_state,
                }) => {
                    if transaction.action == Action::Create {
                       ext.init_code(&params.address, data.to_vec());
                    }

                    ext.update_state()?;

                    Ok(ResultData {
                        gas_left,
                        apply_state: apply_state,
                        data: data.to_vec(),
                        contract: params.address,
                        logs: ext.logs().to_vec(),
                    })
                }
                Err(err) => Err(Error::InternalError(err.to_string())),
            },
            Err(err) => match err {
                vm::TrapError::Call(params, _call) => {
                    Err(Error::InternalError(format!("Error on call: {:?}", params)))
                }
                vm::TrapError::Create(params, address, _create) => Err(Error::InternalError(
                    format!("Error on create at {:?}: {:?}", address, params),
                )),
            },
        }
    }
}
