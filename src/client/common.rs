use tonic::Status;

use crate::{client::GrpcClient, utils};

impl GrpcClient {
    pub fn into_address(bs58_address: &str) -> Result<Vec<u8>, Status> {
        utils::bs58::decode_address(bs58_address).map_err(|e| Status::invalid_argument(e))
    }
}
