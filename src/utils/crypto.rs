use secp256k1::Message;

pub fn hex2sk(pk: &str) -> Result<secp256k1::SecretKey, String> {
    let pk = hex::decode(pk)
        .map_err(|e| e.to_string())?
        .try_into()
        .map_err(|_| "private key convert error".to_string())?;

    Ok(secp256k1::SecretKey::from_byte_array(pk).map_err(|e| e.to_string())?)
}

pub fn sign_tx(txid: Vec<u8>, sk: &secp256k1::SecretKey) -> Result<Vec<u8>, String> {
    let txid: [u8; 32] = txid
        .try_into()
        .map_err(|_| "txid convert error".to_string())?;
    let (rid, sig) = secp256k1::Secp256k1::new()
        .sign_ecdsa_recoverable(Message::from_digest(txid), sk)
        .serialize_compact();
    let mut sig = sig.to_vec();
    sig.push(rid as i32 as u8);
    Ok(sig)
}

#[cfg(test)]
mod test {
    use crate::utils::crypto::{hex2sk, sign_tx};

    #[test]
    fn test_sign_tx() {
        let txid = hex::decode("ef04ae1bba68efc0d43ff95762e202496308b00edd87b70aa2ddde06d0518a97")
            .unwrap();
        let sk =
            hex2sk("399206ef884e86d46f103a76271a2ec33ebe28a65c600379b36e1d2748412a84").unwrap();
        assert_eq!(
            hex::encode(sign_tx(txid, &sk).unwrap()),
            "209cc203bf512c8fd1d6c94821a8a128a36f821a72a7f76bd6750f8c9e7acca52a245338383797c5d239af5d70971ce57d0deb2e9f139ca3700263325d36ab6300"
        )
    }
}
