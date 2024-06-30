#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
//#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);
use zk_logic::{compute_random_bytes, types::{CircuitInputs, CircuitOutputs}};

fn main() {
    let mut input: CircuitInputs = env::read();
    let random_bytes = compute_random_bytes(
        input.public_key.clone(),
        &mut input.nonce,
        &mut input.seed
    );
    let outputs: CircuitOutputs = CircuitOutputs{
        public_key: input.public_key,
        nonce: input.nonce,
        random_bytes
    };
    env::commit(&outputs);
}
