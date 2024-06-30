:warning: This repository describes an experimental consensus protcol that has not yet been used in any network. Proceed with caution! :warning:

# Proof of Random Delta

PoRD is a consensus protocol where the odds of each validator being selected to create the next block are always equal.

Consensus is established over a sequence of commitments (zero knowledge random numbers).

A 2/3rd majority of the known validators must commit to a block for it to be valid.

For more information, read `./whitepaper`.

# Proof of Concept Zero Knowledge Random Number generator for PoRD
To test the zk random number example:
```bash
cd zk-rand
cargo run
```
This will output a zk verifiable random number that was generated with a secret seed.
