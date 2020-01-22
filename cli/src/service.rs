use tonic::{transport::Server, Request, Response, Status};

use contract_deploy::contract_server::{Contract, ContractServer};
use contract_deploy::{DeployReply, DeployRequest};
use contract_deploy::{CallReply, CallRequest};

pub mod contract_deploy {
    tonic::include_proto!("contract_deploy");
}

#[derive(Debug, Default)]
pub struct MyContract {}

#[tonic::async_trait]
impl Contract for MyContract {
    async fn deploy(
        &self,
        request: Request<DeployRequest>,
    ) -> Result<Response<DeployReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = contract_deploy::DeployReply {
            status: format!("Hello {}!", "value").into(),
            hash:format!("Hello {}!", "value").into(),
        };

        Ok(Response::new(reply))
    }

    async fn call_contract(
        &self,
        request: Request<CallRequest>,
    ) -> Result<Response<CallReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = contract_deploy::CallReply {
            status: format!("Hello {}!", "value").into(),
            hash:format!("Hello {}!", "value").into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyContract::default();

    Server::builder()
        .add_service(ContractServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}