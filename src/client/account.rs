use tonic::{Code, Request, Status};

use crate::{client::GrpcClient, tron::protocol::Account, utils::bs58::decode_address};

#[derive(Debug, Copy, Clone)]
pub struct AccountResourceBalance {
    pub bandwidth: i64,
    pub energy: i64,
}

impl GrpcClient {
    pub async fn get_account_trx_balance(&mut self, address: &str) -> Result<i64, Status> {
        let mut req = Request::new(Account::default());
        req.get_mut().address =
            decode_address(address).map_err(|e| Status::new(Code::InvalidArgument, e))?;
        let resp = self.inner.get_account(req).await?;
        Ok(resp.into_inner().balance)
    }

    pub async fn get_account_resource_balance(
        &mut self,
        address: &str,
    ) -> Result<AccountResourceBalance, Status> {
        let mut req = Request::new(Account::default());
        req.get_mut().address =
            decode_address(address).map_err(|e| Status::new(Code::InvalidArgument, e))?;

        let resp = self.inner.get_account_resource(req).await?;
        let res = resp.into_inner();

        Ok(AccountResourceBalance {
            bandwidth: (res.net_limit - res.net_used) + (res.free_net_limit - res.free_net_used),
            energy: res.energy_limit - res.energy_used,
        })
    }
}
