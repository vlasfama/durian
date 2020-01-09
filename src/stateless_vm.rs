use common_types::engines::params::CommonParams;
use ethereum_types::{Address, U256};
use machine::{
    externalities::{OriginInfo, OutputPolicy},
    substate::Substate,
    Machine,
};
use state_cache::StateCache;
use state_provider::StateProvider;
use stateless_ext::StatelessExt;
use std::collections::BTreeMap;
use std::sync::Arc;
use trace::{NoopTracer, NoopVMTracer};
use transaction::Transaction;
use vm::{schedule::WasmCosts, ActionParams, CallType, EnvInfo, Exec, GasLeft};
use wasm::WasmInterpreter;

pub struct StatelessVM {}

impl StatelessVM {
    pub fn new() -> Self {
        StatelessVM {}
    }

    pub fn fire<T: StateProvider>(&self, transaction: Transaction, provider: &mut T) -> Vec<u8>
// TODO:::::::::::: check what to resturn
    /*-> vm::ExecTrapResult<vm::GasLeft>*/ {
        let mut cache = StateCache::new(provider);

        let mut action_params = ActionParams::default();
        action_params.sender = transaction.caller;
        action_params.address = transaction.contract;
        action_params.gas = transaction.gas;
        action_params.data = Some(transaction.data);
        action_params.code = Some(Arc::new(transaction.code));
        /// TODO:????
        let mut deploy = false;

        /// TODO: we can delete this variable
        let mut address11 = Address::zero();
        if action_params.address == Address::zero() {
            let (new_address, _) = match cache.nonce(&transaction.caller) {
                Ok(nonce) => machine::executive::contract_address(
                    vm::CreateContractAddress::FromSenderAndNonce,
                    &transaction.caller,
                    &nonce,
                    &vec![],
                ),

                _ => panic!("invalid address"),
            };

            address11 = new_address;
            action_params.address = new_address;
            deploy = true;
            cache.create_account(address11, U256::zero(), U256::zero(), vec![]);

        }

        let mut env_info = EnvInfo::default();
        env_info.timestamp = 111;
        env_info.gas_limit = U256::from(100000000);
        env_info.number = 1;

        let machine_params = CommonParams::default();
        let builtins = BTreeMap::default();
        let machine = Machine::regular(machine_params, builtins);
        let mut schedule = machine.schedule(env_info.number);
        /// TODO::>?????
        let depth = 100;
        let stack_depth = 100;
        let origin_info = OriginInfo::from(&action_params);
        let mut substate = Substate::new();
        let output = OutputPolicy::InitContract;
        let mut tracer = NoopTracer;
        let mut vm_tracer = NoopVMTracer;
        let static_flag = false;

        let wasm = vm::WasmCosts::default();
        //wasm.have_create2 = true;
        //wasm.have_gasleft = true;
        // TODO:::::::::::::::::
        let mut action_params_1 = action_params.clone();

        schedule.wasm = Some(wasm);
        let mut ext = StatelessExt::new(
            &env_info,
            &machine,
            &schedule,
            //depth,
            //stack_depth,
            action_params_1,
            //&mut substate,
            //output,
            //&mut tracer,
            //&mut vm_tracer,
            //provider,
            //static_flag,
            &mut cache,
        );

        /*
        let (new_address, code_hash) = contract_address(CreateContractAddress::FromSenderAndNonce, &sender, &nonce, &t.data);
                let params = ActionParams {
                    code_address: new_address.clone(),
                    code_hash: code_hash,
                    address: new_address,
                    sender: sender.clone(),
                    origin: sender.clone(),
                    gas: init_gas,
                    gas_price: t.gas_price,
                    value: ActionValue::Transfer(t.value),
                    code: Some(Arc::new(t.data.clone())),
                    code_version: schedule.latest_version,
                    data: None,
                    call_type: CallType::None,
                    params_type: vm::ParamsType::Embedded,
                };

                */


        let interpreter = Box::new(WasmInterpreter::new(action_params));

        let interpreter_return = interpreter
            .exec(&mut ext)
            .ok()
            .expect("Wasm interpreter always calls with trap=false; trap never happens; qed");

            let mut d = vec![1];
        match interpreter_return {
            Ok(ret) => match ret {
                GasLeft::NeedsReturn {
                    gas_left,
                    data,
                    apply_state,
                } => {
                    if deploy == true {
                        d = data.to_vec();
                       // cache.set_code(&address11, data.to_vec());
                    }
                }
                _ => (),
            },
            // TODO
            Err(e) => {
                panic!(e);
            }
        };

        d
    }
}
