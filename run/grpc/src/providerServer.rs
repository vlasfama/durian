use tonic::{transport::Server, Request, Response, Status};

use provider_server::provider_server::{Provider, ProviderServer};
use provider_server::{
    AccountRequest, AccountResponse, BlockRequest, BlockResponse, ContractRequest,
    ContractResponse, ExistRequest, ExistResponse, SetStorageRequest, SetStorageResponse,
    StorageRequest, StorageResponse,
};

pub mod provider_server {
    tonic::include_proto!("provider"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyProvider {}

#[tonic::async_trait]
impl Provider for MyProvider {
    async fn exist(
        &self,
        request: Request<ExistRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<ExistResponse>, Status> {
        // Return an instance of type HelloReply
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

    async fn create_contract(
        &self,
        request: Request<ContractRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<ContractResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = provider_server::ContractResponse {
            message: format!("Hello {}!", request.into_inner().address), // We must use .into_inner() as the fields of gRPC requests and responses are private
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyProvider::default();

    Server::builder()
        .add_service(ProviderServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
