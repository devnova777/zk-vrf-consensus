pub mod types;
use sha2::{Digest, Sha256};
use num_bigint::BigInt;

pub fn compute_random_bytes(public_key: types::PublicKey, nonce: &mut types::Nonce, seed: &mut types::Seed) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    hasher.update(nonce);
    hasher.update(seed);
    let result = hasher.finalize();
    result.to_vec()
}

pub fn random_bytes_to_int(bytes: &Vec<u8>) -> BigInt{
    BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes)
}