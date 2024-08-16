use rand::Rng;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use zk_logic::{
    random_bytes_to_int,
    types::{CircuitInputs, CircuitOutputs},
};
pub use zk_methods::{ZK_RAND_ELF, ZK_RAND_ID};

pub fn generate_random_number(public_key: Vec<u8>, nonce: Vec<u8>) -> Receipt {
    let mut rng = rand::thread_rng();
    let random_float: f64 = rng.gen();
    let seed: Vec<u8> = random_float.to_be_bytes().to_vec();
    let input: CircuitInputs = CircuitInputs {
        public_key,
        // some int .to_bytes()
        nonce,
        // something of sufficient randomness
        seed,
    };
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();
    let prover = default_prover();
    let prove_info = prover.prove(env, ZK_RAND_ELF).unwrap();
    let output: CircuitOutputs = prove_info.receipt.journal.decode().unwrap();
    println!(
        "ZK random number: {:?}",
        random_bytes_to_int(&output.random_bytes)
    );
    //prove_info.receipt.verify(ZK_RAND_ID).unwrap();
    prove_info.receipt
}

#[test]
fn test_generate_random_numbers() {
    for _ in 0..100 {
        let mut rng = rand::thread_rng();
        let random_float: f64 = rng.gen();
        let seed: Vec<u8> = random_float.to_be_bytes().to_vec();
        let input: CircuitInputs = CircuitInputs {
            public_key: vec![0; 32],
            // some int .to_bytes()
            nonce: vec![1; 32],
            // something of sufficient randomness
            seed,
        };
        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();
        let prover = default_prover();
        let prove_info = prover.prove(env, ZK_RAND_ELF).unwrap();
        let output: CircuitOutputs = prove_info.receipt.journal.decode().unwrap();
        println!(
            "ZK random number: {:?}",
            random_bytes_to_int(&output.random_bytes)
        );
        prove_info.receipt.verify(ZK_RAND_ID).unwrap();
    }
}
