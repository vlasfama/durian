extern crate blockchain;
extern crate durian;
use blockchain::blockchain::Blockchain;
use durian::execute;
use durian::transaction::Transaction;
use ethereum_types::{H160, H256, U256};
use provider_server::provider_server::Provider;
use provider_server::{
    AccountRequest, AccountResponse, BlockRequest, BlockResponse, ContractRequest,
    ContractResponse, ExistRequest, ExistResponse, SetStorageRequest, SetStorageResponse,
    StorageRequest, StorageResponse,
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
        request: Request<ExistRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<ExistResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = provider_server::ExistResponse {
            message: format!("Hello {}!", request.into_inner().address), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn storage_at(
        &self,
        request: Request<StorageRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<StorageResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = provider_server::StorageResponse {
            message: format!("Hello {}!", request.into_inner().address), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    // get blockhash
    async fn blockhash(
        &self,
        request: Request<BlockRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<BlockResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = provider_server::BlockResponse {
            message: format!("Hello {}!", request.into_inner().address), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    //create update account
    async fn update_account(
        &self,
        request: Request<AccountRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<AccountResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = provider_server::AccountResponse {
            message: format!("Hello {}!", request.into_inner().address), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    //create contract


    async fn create_contract(
        &self,
        request: Request<ContractRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<ContractResponse>, Status> {
        let mut bc = self.bc.lock().unwrap();


        let from = request.into_inner().from;
        let from_address = H160::from_slice(&from);

        let data = request.into_inner().data;

        let va = request.into_inner().value;
        let value = U256::from_big_endian(&va);

        let gas = request.into_inner().gas;
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
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = provider_server::SetStorageResponse {
            message: format!("Hello {}!", request.into_inner().address), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

}

//deploy the contract
// pub fn create(bc: &mut Blockchain, request: TransactionRequest) -> Result<H256, io::Error> {

// }
