use sha2::{Digest, Sha256};

pub fn decode(data: &str) -> Result<Vec<u8>, String> {
    bs58::decode(data)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_vec()
        .map_err(|e| e.to_string())
}

// base58check: https://developers.tron.network/docs/account#account-address-formats
// tron base58 address = `0x41` + <20 bytes of ethereum address> + <4 bytes of checksum>
pub fn decode_address(address: &str) -> Result<Vec<u8>, String> {
    let data = decode(address)?;
    if data.len() != 25 {
        return Err(format!(
            "invalid address length: get:{}, expect: 25",
            data.len()
        ));
    }
    // check prefix
    if data[0] != 0x41 {
        return Err(format!(
            "invalid address prefix: get:{}, expect: 0x41",
            data[0]
        ));
    }

    let (dec_address, checksum) = data.split_at(21);

    // check sum
    let h = Sha256::digest(dec_address);
    let hh = Sha256::digest(h);

    for (i, v) in checksum.iter().enumerate() {
        if hh[i] != *v {
            return Err(format!(
                "invalid address checksum(index: {}): get:{}, expect:{}",
                i, hh[i], *v
            ));
        }
    }

    return Ok(dec_address.to_vec());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_decode() {
        let data = decode("TE9t1ML5HujuVkGD8qTrWoDbTtMq8LWgzi").expect("decode err");
        let expect = vec![
            65, 45, 229, 56, 247, 28, 142, 228, 82, 105, 139, 145, 155, 250, 82, 122, 160, 121,
            204, 85, 33, 69, 181, 144, 167,
        ];
        assert_eq!(data, expect, "get: {:?}, expect: {:?}", data, expect);

        let data =
            decode_address("TE9t1ML5HujuVkGD8qTrWoDbTtMq8LWgzi").expect("decode address err");
        let expect = vec![
            65, 45, 229, 56, 247, 28, 142, 228, 82, 105, 139, 145, 155, 250, 82, 122, 160, 121,
            204, 85, 33,
        ];
        assert_eq!(data, expect, "get: {:?}, expect: {:?}", data, expect);
    }
}
