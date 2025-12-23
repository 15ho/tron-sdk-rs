use crate::tron::protocol::wallet_client::WalletClient;

#[derive(Debug, Clone)]
pub struct GrpcClient {
    inner: WalletClient<tonic::transport::Channel>,
}

mod common;
pub use common::CreatedAccount;

mod account;
pub use account::AccountResourceBalance;

mod contract;

impl GrpcClient {
    pub async fn new(endpoint: &str) -> Result<Self, tonic::transport::Error> {
        Ok(Self {
            inner: WalletClient::connect(endpoint.to_string()).await?,
        })
    }

    pub fn client(&mut self) -> &mut WalletClient<tonic::transport::Channel> {
        &mut self.inner
    }
}

#[cfg(test)]
pub async fn get_client() -> GrpcClient {
    GrpcClient::new("https://grpc.shasta.trongrid.io:50051")
        .await
        .expect("grpc connect err")
}
