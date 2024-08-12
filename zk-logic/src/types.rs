use serde::{Deserialize, Serialize};

pub type PublicKey = Vec<u8>;
// serialized Nonce
pub type Nonce = Vec<u8>;
// serialized Seed
pub type Seed = Vec<u8>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitInputs {
    pub public_key: PublicKey,
    pub nonce: Nonce,
    pub seed: Seed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitOutputs {
    pub public_key: PublicKey,
    pub nonce: Nonce,
    pub random_bytes: Vec<u8>,
}
