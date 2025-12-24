# Tron SDK Rust

[![Action Status](https://github.com/15ho/tron-sdk-rs/workflows/CI/badge.svg)](https://github.com/15ho/tron-sdk-rs/actions?query=workflow%3ACI)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/15ho/tron-sdk-rs/blob/main/LICENSE)

A comprehensive Rust SDK for the TRON gRPC API, providing seamless integration with TRON full nodes.

## Example
Add dependency in `Cargo.toml`
```toml
[dependencies]
tron-sdk-rs = { git = "https://github.com/15ho/tron-sdk-rs", branch = "main", version = "0.0.1" }
tonic = "0.14"
```
Then, on your `main.rs`
```rust
use tonic::Request;
use tron_sdk_rs::{client::GrpcClient, api::EmptyMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = GrpcClient::new("https://grpc.shasta.trongrid.io:50051").await?;

    // 1. Calling a high-level wrapper method provided by the SDK
    let balance = cli
        .get_account_trx_balance("TXwUd9ywscLUZQcP5tPfqU266kbh3QmYxx")
        .await?;

    println!("trx balance: {}", balance);

    // 2. Calling a raw gRPC method defined in the Tron protocol
    let block = cli
        .client()
        .get_now_block(Request::new(EmptyMessage::default()))
        .await?
        .into_inner();

    if let Some(block_header) = block.block_header {
        if let Some(header_raw) = block_header.raw_data {
            println!("block number: {}", header_raw.number);
        }
    }

    Ok(())
}
```