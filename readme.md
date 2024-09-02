:warning: This repository describes an experimental consensus protcol that has not yet been used in any network. Proceed with caution! :warning:

Review: ZK might not be necessary here as a VRF could suffice. However it does work with ZK random numbers so for now this writeup is not being updated.

# Proof of Random Delta

PoRD is a consensus protocol where the odds of each validator being selected to create the next block are always equal.

It is essentially quite similar to proof of stake (POS) except for a lack of weights / crypto economic incentive.

Consensus is established over a sequence of commitments (zero knowledge random numbers or VRF).

A 51%+ majority of the known validators must commit to a block for it to be valid.

For more information, read `./whitepaper`.

This crate exposes `zk-rand` a zero knowledge, verifiable random function built with Risc0.