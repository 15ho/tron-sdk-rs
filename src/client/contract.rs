use num_bigint::BigInt;
use tonic::{Code, Request, Response, Status};

use crate::{
    client::GrpcClient,
    tron::protocol::{TransactionExtention, TriggerSmartContract},
    utils::bigint,
};

impl GrpcClient {
    pub async fn contract_call(
        &mut self,
        from: Option<&str>,
        contract: &str,
        call_data: String,
        writable: Option<i64>, // fee limit
    ) -> Result<Response<TransactionExtention>, Status> {
        let mut req = Request::new(TriggerSmartContract::default());
        if let Some(from_address) = from {
            req.get_mut().owner_address = Self::into_address(from_address)?;
        }
        req.get_mut().contract_address = Self::into_address(contract)?;
        req.get_mut().data = hex::decode(call_data)
            .map_err(|e| Status::new(Code::InvalidArgument, e.to_string()))?;

        if let Some(fee_limit) = writable {
            self.inner.trigger_contract(req).await.map(|mut resp| {
                let ext = resp.get_mut();
                if let Some(raw) = ext.transaction.as_mut().and_then(|tx| tx.raw_data.as_mut()) {
                    raw.fee_limit = fee_limit;
                }
                resp
            })
        } else {
            self.inner.trigger_constant_contract(req).await
        }
    }

    pub async fn trc20_balance(&mut self, from: &str, contract: &str) -> Result<BigInt, Status> {
        let from_address = Self::into_address(from)?;
        // https://learnevm.com/chapters/abi-encoding/anatomy#the-anatomy-of-an-abi-encoded-function-call
        // call data = <function selector> + <parameters>
        // function balanceOf(address _owner) public view returns (uint256 balance)
        let call_data = format!("70a08231{:0>64}", hex::encode(from_address));
        let resp = self
            .contract_call(Some(from), contract, call_data, None)
            .await?;
        let call_res = resp.into_inner().constant_result;
        if call_res.len() != 1 {
            return Err(Status::internal(format!(
                "constant result({:?}) length is not one",
                call_res
            )));
        }
        Ok(bigint::from_bytes(&call_res[0]))
    }

    pub async fn trc20_transfer(
        &mut self,
        from: &str,
        to: &str,
        contract: &str,
        amount: BigInt,
        fee_limit: i64,
    ) -> Result<Response<TransactionExtention>, Status> {
        let to_address = Self::into_address(to)?;

        // function transfer(address _to, uint256 _value) public returns (bool success)
        let call_data = format!(
            "a9059cbb{:0>64}{:0>64}",
            hex::encode(to_address),
            hex::encode(amount.to_string())
        );

        self.contract_call(Some(from), contract, call_data, Some(fee_limit))
            .await
    }
}

#[cfg(test)]
mod test {
    use num_bigint::BigInt;

    use crate::client::get_client;

    #[tokio::test]
    async fn test_trc20_balance() {
        let mut cli = get_client().await;
        let balance = cli
            .trc20_balance(
                "TD614vXXu2qH3VrMnkeKJSW4q7FUMVstar",
                "TTvFE3RWS48yNhS6EaK3SGbBYycaEmV7Pw",
            )
            .await
            .expect("get trc20 balance err");
        assert_eq!(BigInt::from(123455e6 as i64), balance)
    }

    #[tokio::test]
    async fn test_trc20_transfer() {
        let mut cli = get_client().await;
        let ext = cli
            .trc20_transfer(
                "TXwUd9ywscLUZQcP5tPfqU266kbh3QmYxx",
                "TD614vXXu2qH3VrMnkeKJSW4q7FUMVstar",
                "TTvFE3RWS48yNhS6EaK3SGbBYycaEmV7Pw",
                BigInt::from(100),
                100000,
            )
            .await
            .expect("create trc20 transfer tx err");

        // TODO: assert
        println!("trc20 transfer tx: {:?}", ext);
    }
}
