use std::{sync::Arc};
use ethereum_types::{Address, H256, U256};
use parity_bytes::Bytes;
use machine::{
    externalities::{OutputPolicy},
    substate::Substate,
    Machine,
};
use state_cache::StateCache;
use trace::{Tracer, VMTracer};
use vm::{
    self, CallType, ContractCreateResult, ActionParams, CreateContractAddress,
    EnvInfo, Ext, MessageCallResult, ReturnData, Schedule, TrapKind,
};
use state_provider::StateProvider;



pub struct StatelessExt<'a/*, T: 'a, V: 'a, S:'a*/> {
    env_info: &'a EnvInfo,
    //depth: usize,
    //stack_depth: usize,
    params: &'a ActionParams,
    //substate: &'a mut Substate,
    machine: &'a Machine,
    schedule: &'a Schedule,
    //output: OutputPolicy,
    //tracer: &'a mut T,
    //vm_tracer: &'a mut V,
    //state_provider: &'a S,
    //static_flag: bool,
  //  storageProvider: StateProvider;
    cache:  StateCache<'a>,
}

impl<'a/*, T: 'a, V: 'a, S: 'a*/> StatelessExt<'a/*, T, V, S*/>
/*where
    T: Tracer,
    V: VMTracer,
    S: StateProvider,
*/
{
    pub fn new(
        env_info: &'a EnvInfo,
        machine: &'a Machine,
        schedule: &'a Schedule,
        //depth: usize,
        //stack_depth: usize,
		params: &'a ActionParams,
        //substate: &'a mut Substate,
        //output: OutputPolicy,
        //tracer: &'a mut T,
        //vm_tracer: &'a mut V,
        provider: &'a mut dyn StateProvider,
        //static_flag: bool,
    ) -> Self {

        let mut cache = StateCache::new(provider);

        StatelessExt {
            env_info,
            //depth,
            //stack_depth,
            params,
            //substate,
            machine,
            schedule,
            //output,
            //tracer,
            //vm_tracer,
            //static_flag,
            cache,
        }
    }
}

impl<'a> Ext for StatelessExt<'a>
{
    fn initial_storage_at(&self, key: &H256) -> vm::Result<H256> {
        println!("hello");
        /*if self.state.is_base_storage_root_unchanged(&self.origin_info.address)? {
            self.state.checkpoint_storage_at(0, &self.origin_info.address, key).map(|v| v.unwrap_or_default()).map_err(Into::into)
        } else {
            warn!(target: "externalities", "Detected existing account {:#x} where a forced contract creation happened.", self.origin_info.address);
            Ok(H256::zero())
        }*/

        Ok(H256::zero())
    }

    fn storage_at(&self, key: &H256) -> vm::Result<H256> {
        self.cache.storage_at(&self.params.address, key).map_err(Into::into)
    }

    fn set_storage(&mut self, key: H256, value: H256) -> vm::Result<()> {
        self.cache.set_storage(&self.params.address, &key, &value);
        Ok(())
    }

    fn exists(&self, address: &Address) -> vm::Result<bool> {
        println!("hello");
        //self.state.exists(address).map_err(Into::into)
        Ok(true)
    }

    fn exists_and_not_null(&self, address: &Address) -> vm::Result<bool> {
        println!("hello");
        //self.state.exists_and_not_null(address).map_err(Into::into)
        Ok(true)
    }

    fn origin_balance(&self) -> vm::Result<U256> {
        println!("hello");
        //self.balance(&self.origin_info.address).map_err(Into::into)
        Ok(U256::zero())
    }

    fn balance(&self, address: &Address) -> vm::Result<U256> {
        println!("hello");
        //self.state.balance(address).map_err(Into::into)
        Ok(U256::zero())
    }

    fn blockhash(&mut self, number: &U256) -> H256 {
        H256::zero()
    }

    fn create(
        &mut self,
        gas: &U256,
        value: &U256,
        code: &[u8],
        parent_version: &U256,
        address_scheme: CreateContractAddress,
        trap: bool,
    ) -> ::std::result::Result<ContractCreateResult, TrapKind> {
        println!("hello");
        Ok(ContractCreateResult::Failed)
    }

    fn call(
        &mut self,
        gas: &U256,
        sender_address: &Address,
        receive_address: &Address,
        value: Option<U256>,
        data: &[u8],
        code_address: &Address,
        call_type: CallType,
        trap: bool,
    ) -> ::std::result::Result<MessageCallResult, TrapKind> {
        println!("hello");
        Ok(MessageCallResult::Failed)
    }

    fn extcode(&self, address: &Address) -> vm::Result<Option<Arc<Bytes>>> {
        println!("hello");
        Ok(None)
    }

    fn extcodehash(&self, address: &Address) -> vm::Result<Option<H256>> {
        Ok(None)
    }

    fn extcodesize(&self, address: &Address) -> vm::Result<Option<usize>> {
        Ok(None)
    }

    fn log(&mut self, topics: Vec<H256>, data: &[u8]) -> vm::Result<()> {
        println!("hello");
        Ok(())
    }

    fn ret(self, gas: &U256, data: &ReturnData, apply_state: bool) -> vm::Result<U256>
    where
        Self: Sized,
    {
        Ok(U256::zero())
    }

    fn suicide(&mut self, refund_address: &Address) -> vm::Result<()> {
        println!("hello");
        Ok(())
    }

    fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    fn env_info(&self) -> &EnvInfo {
        self.env_info
    }

    fn chain_id(&self) -> u64 {
        self.machine.params().chain_id
    }

    fn depth(&self) -> usize {
        //self.depth
        1
    }

    fn add_sstore_refund(&mut self, value: usize) {
        //self.substate.sstore_clears_refund += value as i128;
    }

    fn sub_sstore_refund(&mut self, value: usize) {
        //self.substate.sstore_clears_refund -= value as i128;
    }

    fn trace_next_instruction(&mut self, pc: usize, instruction: u8, current_gas: U256) -> bool {
        //self.vm_tracer
        //    .trace_next_instruction(pc, instruction, current_gas)
    true
    }

    fn trace_prepare_execute(
        &mut self,
        pc: usize,
        instruction: u8,
        gas_cost: U256,
        mem_written: Option<(usize, usize)>,
        store_written: Option<(U256, U256)>,
    ) {
        //self.vm_tracer
        //    .trace_prepare_execute(pc, instruction, gas_cost, mem_written, store_written)
    }

    fn trace_failed(&mut self) {
        //self.vm_tracer.trace_failed();
    }

    fn trace_executed(&mut self, gas_used: U256, stack_push: &[U256], mem: &[u8]) {
        //self.vm_tracer.trace_executed(gas_used, stack_push, mem)
    }

    fn is_static(&self) -> bool {
        //return self.static_flag;
        return false;
    }


}
