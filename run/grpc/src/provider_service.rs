extern crate blockchain;
extern crate durian;
use blockchain::blockchain::Blockchain;
use durian::execute;
use durian::provider::Provider as durian_provider;
use durian::transaction::Transaction;
use ethereum_types::{H160, H256, U256};
use provider_server::provider_server::Provider;

use provider_server::{
    AccountRequest, AccountResponse, AuthorResponse, BlockhashRequest, BlockhashResponse,
    BlocknumberResponse, ContractRequest, ContractResponse, DifficultyResponse, Empty,
    ExistRequest, ExistResponse, GasResponse, SetStorageRequest, SetStorageResponse,
    StorageRequest, StorageResponse, TimestampRequest, TimpestampResponse, UpdateRequest,
    UpdateResponse,
};
use std::sync::Mutex;
use tonic::{Request, Response, Status};

pub mod provider_server {
    tonic::include_proto!("provider"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct MyProvider {
    bc: Mutex<Blockchain>,
}

impl MyProvider {
    pub fn new(bc: Blockchain) -> Self {
        MyProvider { bc: Mutex::new(bc) }
    }
}

#[tonic::async_trait]
impl Provider for MyProvider {
    async fn exist(
        &self,
        request: Request<ExistRequest>,
    ) -> Result<Response<ExistResponse>, Status> {
        let mut bc = self.bc.lock().unwrap();
        let msg = request.into_inner();

        let addr = msg.address;
        let address = H160::from_slice(&addr);

        //call the exist method
        let result = &bc.exist(&address);
        let reply = provider_server::ExistResponse { message: *result };

        Ok(Response::new(reply))
    }

    async fn storage_at(
        &self,
        request: Request<StorageRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<StorageResponse>, Status> {
        let msg = request.into_inner();

        let addr = msg.address;
        let address = H160::from_slice(&addr);

        let key = msg.key;
        let key_addr = H256::from_slice(&key);

        let mut bc = self.bc.lock().unwrap();
        let value = &bc.storage_at(&address, &key_addr);

        let result = value.unwrap().as_bytes().to_vec();

        let reply = provider_server::StorageResponse { message: result };

        Ok(Response::new(reply))
    }

    // get blockhash
    async fn blockhash(
        &self,
        request: Request<BlockhashRequest>,
    ) -> Result<Response<BlockhashResponse>, Status> {
        let msg = request.into_inner();
        let num = msg.num;

        let mut bc = self.bc.lock().unwrap();
        let value = &bc.block_hash(num);
        let result = value.unwrap();
        let reply = provider_server::BlockhashResponse {
            message: result.as_bytes().to_vec(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    //create update account
    async fn update_account(
        &self,
        request: Request<UpdateRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UpdateResponse>, Status> {
        let msg = request.into_inner();

        let addr = msg.address;
        let address = H160::from_slice(&addr);

        let bal = msg.balance;
        let balance = U256::from_big_endian(&bal);

        let nc = msg.nonce;
        let nonce = U256::from_big_endian(&nc);

        let mut bc = self.bc.lock().unwrap();
        let value = &bc.update_account(&address, &balance, &nonce);

        let result = true;
        let reply = provider_server::UpdateResponse {
            message: result, // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    //create contract
    async fn create_contract(
        &self,
        request: Request<ContractRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<ContractResponse>, Status> {
        //instance of blockchain
        let mut bc = self.bc.lock().unwrap();
        let msg = request.into_inner();

        let from = msg.from;
        let from_address = H160::from_slice(&from);

        let data = msg.data;
        let va = msg.value;
        let value = U256::from_big_endian(&va);

        let gas = msg.gas;
        let gas_value = U256::from_big_endian(&gas);
        let code = data;
        bc.commit();
        let tx1 = Transaction::make_create_embedded_code(
            from_address,
            U256::zero(),
            value,
            gas_value,
            code,
            H256::zero(),
        );

        let ret1 = execute::execute(&tx1.clone(), &mut bc.clone()).unwrap();
        let tx_hash = bc.add_transactions(tx1, ret1);
        let hash = tx_hash.as_bytes();
        bc.inc_nonce("naga");
        bc.commit();

        let reply = provider_server::ContractResponse {
            txhash: hash.to_vec(),
        };
        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn set_storage(
        &self,
        request: Request<SetStorageRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<SetStorageResponse>, Status> {
        let msg = request.into_inner();

        let addr = msg.address;
        let address = H160::from_slice(&addr);

        let key = msg.key;
        let _key = H256::from_slice(&key);

        let val = msg.value;
        let value = H256::from_slice(&val);

        let mut bc = self.bc.lock().unwrap();
        let _value = &bc.set_storage(&address, &_key, &value);
        let result = true;
        let reply = provider_server::SetStorageResponse {
            message: result, // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn account(
        &self,
        request: Request<AccountRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<AccountResponse>, Status> {
        let msg = request.into_inner();

        let addr = msg.address;
        let address = H160::from_slice(&addr);

        let mut bc = self.bc.lock().unwrap();
        let _value = &bc.account(&address);

        let bal = _value.unwrap().balance;

        let code = _value.unwrap().code;
        let nonce = _value.unwrap().nonce;
        let reply = provider_server::AccountResponse {
            balance: bal,
            code: code,
            nonce: nonce, // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn timestamp(
        &self,
        request: Request<TimestampRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<TimpestampResponse>, Status> {
        let msg = request.into_inner();

        let mut bc = self.bc.lock().unwrap();
        let value = &bc.timestamp();
        let reply = provider_server::TimpestampResponse { message: *value };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn block_number(
        &self,
        request: Request<Empty>, // Accept request of type HelloRequest
    ) -> Result<Response<BlocknumberResponse>, Status> {
        let msg = request.into_inner();

        let mut bc = self.bc.lock().unwrap();
        let value = &bc.block_number();
        let reply = provider_server::BlocknumberResponse { message: *value };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn block_author(
        &self,
        request: Request<Empty>, // Accept request of type HelloRequest
    ) -> Result<Response<AuthorResponse>, Status> {
        let msg = request.into_inner();

        let mut bc = self.bc.lock().unwrap();
        let value = &bc.block_author();
        let result = value.unwrap().as_bytes().to_vec();
        let reply = provider_server::AuthorResponse { message: result };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn difficulty(
        &self,
        request: Request<Empty>, // Accept request of type HelloRequest
    ) -> Result<Response<DifficultyResponse>, Status> {
        let msg = request.into_inner();
        let mut bc = self.bc.lock().unwrap();
        let value = &bc.difficulty();
        let result = value.unwrap();
        let reply = provider_server::DifficultyResponse { message: result };
        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn gas_limit(
        &self,
        request: Request<Empty>, // Accept request of type HelloRequest
    ) -> Result<Response<GasResponse>, Status> {
        let msg = request.into_inner();
        let mut bc = self.bc.lock().unwrap();
        let value = &bc.gas_limit();
        let result = value.unwrap();
        let reply = provider_server::GasResponse { gas: result };
        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
