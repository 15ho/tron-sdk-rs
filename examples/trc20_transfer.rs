use std::{error::Error, str::FromStr};

use clap::Parser;
use num_bigint::BigInt;
use tonic::Request;
use tron_sdk_rs::{
    client::{Address, GrpcClient},
    utils::crypto,
};

#[derive(Parser, Debug)]
struct Args {
    /// Tron FullNode gRPC endpoint
    #[arg(short, long, default_value = "https://grpc.shasta.trongrid.io:50051")]
    endpoint: String,

    /// Tron account private key(hex format)
    #[arg(env = "TRON_PRIVATE_KEY", hide = true)]
    private_key: String,

    /// TRC20 contract address
    #[arg(short, long)]
    contract: String,

    /// TRC20 receiver address
    #[arg(short, long)]
    to: String,

    /// Transfer amount
    #[arg(short, long)]
    amount: String,

    /// Transaction fee limit
    #[arg(short, long, default_value_t = 100e6 as i64)]
    fee_limit: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let amount = BigInt::from_str(&args.amount)?;
    println!("transfer amount: {}", amount);

    let sk = crypto::hex2sk(&args.private_key)?;

    let address = Address::from_public_key(&sk.public_key(&secp256k1::Secp256k1::new()));

    let from = address.to_bs58();
    println!("from address: {}", from);

    let mut cli = GrpcClient::new(&args.endpoint).await?;
    let ext = cli
        .trc20_transfer(&from, &args.to, &args.contract, amount, args.fee_limit)
        .await?
        .into_inner();
    let mut tx = ext.transaction.ok_or("create transfer tx error")?;
    tx.signature.push(crypto::sign_tx(ext.txid.clone(), &sk)?);

    println!("tx hash: {}", hex::encode(ext.txid));

    let res = cli
        .client()
        .broadcast_transaction(Request::new(tx))
        .await?
        .into_inner();
    println!(
        "send tx result: success={}, message={}",
        res.result,
        String::from_utf8(res.message)
            .map_err(|e| format!("result message stringify err: {:?}", e))?,
    );

    Ok(())
}
