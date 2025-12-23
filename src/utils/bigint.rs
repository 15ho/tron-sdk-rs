use num_bigint::BigInt;

pub fn from_bytes(bytes: &[u8]) -> BigInt {
    BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes)
}
