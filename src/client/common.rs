use prost::Message;
use secp256k1::{Secp256k1, rand};
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use tonic::Status;

use crate::{
    api,
    client::GrpcClient,
    utils::{self, bs58},
};

#[derive(Debug, Clone)]
pub struct CreatedAccount {
    pub private_key_hex: String,
    pub address_bs58: String,
}

#[derive(Debug, Clone)]
pub struct Address(Vec<u8>);

impl Address {
    pub fn from_bs58(address: &str) -> Result<Self, String> {
        Ok(Self(utils::bs58::decode_address(address)?))
    }

    pub fn from_public_key(public_key: &secp256k1::PublicKey) -> Address {
        Address(
            Keccak256::digest(&public_key.serialize_uncompressed().to_vec()[1..])[12..].to_vec(),
        )
    }

    pub fn into_inner(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn to_bs58(&self) -> String {
        utils::bs58::encode_address(self.0.clone())
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0.clone())
    }

    pub fn to_hex_with_prefix(&self) -> String {
        let mut h = self.to_hex();
        h.insert_str(0, "0x");
        h
    }
}

impl GrpcClient {
    pub fn parse_address(address: &str) -> Result<Address, Status> {
        Address::from_bs58(address)
            .map_err(|e| Status::invalid_argument(format!("parse address err: {}", e)))
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

    pub fn get_tx_hash(tx_raw: &api::transaction::Raw) -> Vec<u8> {
        Sha256::digest(tx_raw.encode_to_vec()).to_vec()
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
