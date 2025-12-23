use tonic::{Request, Status};

use crate::{client::GrpcClient, tron::protocol::Account};

#[derive(Debug, Copy, Clone)]
pub struct AccountResourceBalance {
    pub bandwidth: i64,
    pub energy: i64,
}

impl GrpcClient {
    pub async fn get_account_trx_balance(&mut self, address: &str) -> Result<i64, Status> {
        let mut req = Request::new(Account::default());
        req.get_mut().address = Self::into_address(address)?;
        let resp = self.inner.get_account(req).await?;
        Ok(resp.into_inner().balance)
    }

    pub async fn get_account_resource_balance(
        &mut self,
        address: &str,
    ) -> Result<AccountResourceBalance, Status> {
        let mut req = Request::new(Account::default());
        req.get_mut().address = Self::into_address(address)?;

        let resp = self.inner.get_account_resource(req).await?;
        let res = resp.into_inner();

        Ok(AccountResourceBalance {
            bandwidth: (res.net_limit - res.net_used) + (res.free_net_limit - res.free_net_used),
            energy: res.energy_limit - res.energy_used,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::client::get_client;

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
