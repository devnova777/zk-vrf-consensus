# Proof of Random Delta

PoRD is a consensus protocol where the odds of each validator to be selected to create the next block are always equal.

Consensus is established over a sequence of commitments (zero knowledge random numbers).

A 2/3rd majority of the known validators must commit to a block for it to be valid.

For more information, read `./whitepaper`.

# Proof of Concept Zero Knowledge Random Number generator fot PoRD
To test the zk random number example:
```bash
cd zk-rand
cargo run
```
This will output a zk verifiable random number that was generated with a secret seed.