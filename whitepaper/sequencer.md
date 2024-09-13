# Whitepaper: L2 Distributed Sequencer
Design and Implementation by [Jonas Pauli](https://www.linkedin.com/in/jonas-pauli/)
## Introduction
This paper describes a distributed sequencer for Layer 2 Blockchain Transactions. Decentralized sequencing is difficult to achieve and usually involves multiple consensus layers and is reliant complex tokenomics. This project strives to offer a simplified alternative to centralized sequencing with a fixed set of distributed validators rather than achieve the highest possible degree of decentralization. By design this sequencer has *no tokenomic model* but in theory one could be built on top of the simple consensus layer.
## Consensus
Validator Nodes collect arbitrary Transactions, or better said arbitrary Transaction Hashes and store them in a temporary mempool. As for the first iteration of development, the *MVP*, transaction pools are not synchronized between nodes but instead each node commits its pool when proposing a Block. This is not ideal but assuming a small set of validators can be sufficiently fast. 

For each Block height *h* a round *r* is defined as:

```rust
    let round = (get_current_time() - last_block_unix_timestamp)
        / ROUND_DURATION;
```

A block is stored when a round concludes successfully. There can be multiple rounds for a single block height in case the selected validator fails to commit a random number for a round.

In cases where the round validator fails to commit a random number by the end of the round, a new validator will be selected (the next validator in the fixed set of validators). The validator chosen to commit a random number is chosen such that:

```rust
    let committing_validator = validators[round % (validators.len() - 1)]
```

The selected round validator is eligble of committing a random value to other nodes. Those nodes will then deterministically evaluate the validator who is eligible of creating the next block:

```rust
    let index = (validator_commitment % (validators.len() - 1))
    let proposing_validator = validators[index as usize]
```