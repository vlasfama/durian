use common_types::engines::params::CommonParams;
use ethereum_types::{Address, U256};
use machine::{
    externalities::{OriginInfo, OutputPolicy},
    substate::Substate,
    Machine,
};
use state_provider::{Error, StateProvider};
use stateless_ext::StatelessExt;
use std::collections::BTreeMap;
use std::sync::Arc;
use trace::{NoopTracer, NoopVMTracer};
use transaction::{Action, Transaction};
use vm::{ActionParams, ActionValue, CallType, EnvInfo, Exec, GasLeft, ParamsType};
use wasm::WasmInterpreter;

pub struct StatelessVM {}

impl StatelessVM {
    pub fn new() -> Self {
        StatelessVM {}
    }

    pub fn fire<T: StateProvider>(
        &self,
        transaction: Transaction,
        provider: &mut T,
    ) /*-> vm::ExecTrapResult<GasLeft>*/ {
        let params = match transaction.action {
            Action::Create => {
                let acc = match provider.account(&transaction.sender) {
                    Ok(val) => val,
                    Err(err) => panic!("Invalid sender"),
                };
                let (new_address, _) = machine::executive::contract_address(
                    vm::CreateContractAddress::FromSenderAndNonce,
                    &transaction.sender,
                    &acc.nonce,
                    &vec![],
                );

                provider.create_contract(new_address, U256::from(1));

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
            Action::Call(ref address) => ActionParams {
                code_address: address.clone(),
                address: address.clone(),
                sender: transaction.sender.clone(),
                origin: transaction.sender.clone(),
                gas: transaction.gas,
                value: ActionValue::Transfer(transaction.value),
                code: Some(Arc::new(transaction.code)),
                data: Some(transaction.params),
                call_type: CallType::Call,
                params_type: ParamsType::Separate,
                gas_price: U256::zero(),
                code_hash: None,
                code_version: U256::zero(),
            },
        };

        let mut env_info = EnvInfo::default();
        env_info.timestamp = 111;
        env_info.gas_limit = U256::from(100000000);
        env_info.number = 1;

        let machine_params = CommonParams::default();
        let builtins = BTreeMap::default();
        let machine = Machine::regular(machine_params, builtins);
        let mut schedule = machine.schedule(env_info.number);

        let wasm = vm::WasmCosts::default();
        schedule.wasm = Some(wasm);

        let mut ext = StatelessExt::new(&env_info, &machine, &schedule, &params, provider);

        let interpreter = Box::new(WasmInterpreter::new(params.clone()));

        let ret = interpreter.exec(&mut ext);
        match ret {
            Ok(val) => {
                match val {
                    Ok(GasLeft::Known(gas_left)) => {
                        /*Ok(FinalizationResult {
                            gas_left,
                            apply_state: true,
                            return_data: ReturnData::empty()
                        })
                        */
                    },
                    Ok(GasLeft::NeedsReturn { gas_left, data, apply_state }) => {
                        if transaction.action == Action::Create {
                            provider.init_code(&params.address, data.to_vec());

                        }
                    },
                    Err(err) => panic!("error {}", err),
                }
            }
            Err(err) => panic!("error"),
        }



    }
}
