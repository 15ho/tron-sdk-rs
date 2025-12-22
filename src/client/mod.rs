use crate::tron::protocol::wallet_client::WalletClient;

#[derive(Debug, Clone)]
struct GrpcClient {
    inner: WalletClient<tonic::transport::Channel>,
}

mod account;

impl GrpcClient {
    pub async fn new(endpoint: &str) -> Result<GrpcClient, tonic::transport::Error> {
        Ok(GrpcClient {
            inner: WalletClient::connect(endpoint.to_string()).await?,
        })
    }

    pub fn client(&mut self) -> &mut WalletClient<tonic::transport::Channel> {
        &mut self.inner
    }
}

#[cfg(test)]
mod test {
    use tokio::{self, sync::OnceCell};

    use crate::{client::GrpcClient, tron::protocol::Account, utils};
    use tonic::Request;

    static GRPC_CLIENT: OnceCell<GrpcClient> = OnceCell::const_new();

    async fn get_client() -> GrpcClient {
        GRPC_CLIENT
            .get_or_init(|| async {
                GrpcClient::new("https://grpc.shasta.trongrid.io:50051")
                    .await
                    .expect("grpc connect err")
            })
            .await
            .clone()
    }

    #[tokio::test]
    async fn test_new_grpc_client() {
        // https://developers.tron.network/docs/networks#shasta-testnet
        let mut cli = get_client().await;
        let mut acc_req = Account::default();
        acc_req.address = utils::bs58::decode_address("TE9t1ML5HujuVkGD8qTrWoDbTtMq8LWgzi")
            .expect("decode address err");
        let resp = cli
            .client()
            .get_account(Request::new(acc_req))
            .await
            .expect("get balance err");

        println!("balance resp: {:?}", resp);
    }

    #[tokio::test]
    async fn test_get_account_trx_balance() {
        let mut cli = get_client().await;
        let trx_balance = cli
            .get_account_trx_balance("TE9t1ML5HujuVkGD8qTrWoDbTtMq8LWgzi")
            .await
            .expect("get account trx balance err");

        // TODO: assert
        println!("trx balance: {}", trx_balance);
    }

    #[tokio::test]
    async fn test_get_account_resource_balance() {
        let mut cli = get_client().await;
        let res_balance = cli
            .get_account_resource_balance("TFysCB929XGezbnyumoFScyevjDggu3BPq")
            .await
            .expect("get account trx balance err");

        // TODO: assert
        println!("resource balance: {:?}", res_balance);
    }
}
