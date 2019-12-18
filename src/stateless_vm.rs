use common_types::engines::params::CommonParams;
use ethereum_types::{Address, U256};
use machine::{
    externalities::{OriginInfo, OutputPolicy},
    substate::Substate,
    Machine,
};
use state_provider::StateProvider;
use stateless_ext::StatelessExt;
use std::collections::BTreeMap;
use trace::{NoopTracer, NoopVMTracer};
use transaction::Transaction;
use vm::{schedule::WasmCosts, ActionParams, EnvInfo, Exec, CallType, GasLeft};
use wasm::WasmInterpreter;

pub struct StatelessVM {}

impl StatelessVM {
    pub fn new() -> Self {
        StatelessVM {}
    }

    pub fn fire<T: StateProvider>(
        &self,
        transaction: Transaction,
        provider: &T,
    ) -> vm::ExecTrapResult<vm::GasLeft> {
        let mut action_params = ActionParams::default();
        ////action_params.call_type =  CallType::CallCode;
        action_params.sender = transaction.caller;
        action_params.origin = transaction.caller;
        action_params.address = transaction.caller;
        action_params.gas = transaction.gas;
        action_params.data = transaction.data;
        action_params.code = transaction.code;

        let mut env_info = EnvInfo::default();
        env_info.timestamp = 111;
        env_info.gas_limit = U256::from(100000000);
        env_info.number = 1;
        let machine_params = CommonParams::default();
        let builtins = BTreeMap::default();
        let machine = Machine::regular(machine_params, builtins);
        let mut schedule = machine.schedule(env_info.number);
        /// TODO::>?????
        let depth = 10;
        let stack_depth = 10;
        //let origin_info = OriginInfo::from(&action_params);
        let mut substate = Substate::new();
        let output = OutputPolicy::InitContract;
        let mut tracer = NoopTracer;
        let mut vm_tracer = NoopVMTracer;
        let static_flag = false;

        let mut wasm = vm::WasmCosts::default();
        wasm.have_create2 = true;
        wasm.have_gasleft = true;

        schedule.wasm = Some(wasm);
        let mut ext = StatelessExt::new(
            &env_info,
            &machine,
            &schedule,
            //depth,
            //stack_depth,
            ////&origin_info,
            //&mut substate,
            //output,
            //&mut tracer,
            //&mut vm_tracer,
            //provider,
            //static_flag,
        );

        let interpreter = Box::new(WasmInterpreter::new(action_params));

        interpreter.exec(&mut ext)
    }
}
