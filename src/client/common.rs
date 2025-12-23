use secp256k1::{Secp256k1, rand};
use sha2::Digest;
use sha3::Keccak256;
use tonic::Status;

use crate::{
    client::GrpcClient,
    utils::{self, bs58},
};

#[derive(Debug, Clone)]
pub struct CreatedAccount {
    pub private_key_hex: String,
    pub address_bs58: String,
}

impl GrpcClient {
    pub fn into_address(bs58_address: &str) -> Result<Vec<u8>, Status> {
        utils::bs58::decode_address(bs58_address).map_err(|e| Status::invalid_argument(e))
    }

    // https://developers.tron.network/docs/account#externally-owned-account-creation
    pub fn create_account() -> CreatedAccount {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());

        CreatedAccount {
            private_key_hex: hex::encode(secret_key.secret_bytes()),
            address_bs58: bs58::encode_address(
                Keccak256::digest(&public_key.serialize_uncompressed().to_vec()[1..])[12..]
                    .to_vec(),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::client::GrpcClient;

    #[test]
    fn test_create_account() {
        println!("account: {:?}", GrpcClient::create_account());
        println!("account2: {:?}", GrpcClient::create_account());
    }
}
