extern crate blockchain;
mod provider_service;
use blockchain::blockchain::Blockchain;
use provider_server::provider_server::ProviderServer;
use provider_service::{provider_server, MyProvider};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let bc = Blockchain::new();
    let provider = MyProvider::new(bc);

    Server::builder()
        .add_service(ProviderServer::new(provider))
        .serve(addr)
        .await?;

    Ok(())
}
