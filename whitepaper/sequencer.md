# Whitepaper: L2 Distributed Sequencer
Design and Implementation by [Jonas Pauli](https://www.linkedin.com/in/jonas-pauli/)
## Introduction
This paper describes a distributed sequencer for Layer 2 Blockchain Transactions. Decentralized sequencing is difficult to achieve and usually involves multiple consensus layers and is reliant complex tokenomics. This project strives to offer a simplified alternative to centralized sequencing with a fixed set of distributed validators rather than achieve the highest possible degree of decentralization. By design this sequencer has *no tokenomic model* but in theory one could be built on top of the simple consensus layer.

## Implementation details: Work in Progress
An *MVP* (Minimum Viable Product) implementation of this sequencer can be found [here](https://github.com/jonas089/l2-sequencer).
This repository includes a docker compose that helps quickly setup a network of *4* nodes. This has however only been tested
on an M2 Macbook Pro and x86 support is still in the works. Also due to ongoing work there might be new instability introduced.
I will update the status below to reflect the stability of the codebase:

```
Stability: 

*LOW
MEDIUM
HIGH, 

last updated: Friday, September 13
```


## Liveness guarantees
Firstly it is important to note that this codebase is provided as-is and there are no guarantees for its functionality at this point in time.
In order to improve liveness several design decisions have been made and a minimum viable consensus protocol has been designed.

### Consensus
Validator Nodes collect arbitrary Transactions, or better said arbitrary Transaction Hashes and store them in a temporary mempool. As for the first iteration of development, the *MVP*, transaction pools are not synchronized between nodes but instead each node commits its pool when proposing a Block. This is not ideal but assuming a small set of validators can be sufficiently fast. 

For each Block height *h* a round *r* is defined as:

```rust
    let round = (get_current_time() - last_block_unix_timestamp)
        / ROUND_DURATION;
```

A block is stored when a round concludes successfully. There can be multiple rounds for a single block height in case the selected validator fails to commit a random number for a round.

In cases where the `committing_validator` fails to commit a random number by the end of the round, a new validator will be selected (the next validator in the fixed set of validators). The validator chosen to commit a random number is chosen such that:

```rust
    let committing_validator = validators[round % (validators.len() - 1)]
```

The selected `committing_validator` is eligble of committing a verifiable random value (generated using a VRF - currently a ZK VRF) to other nodes. Those nodes will then deterministically evaluate the validator who is eligible of creating the next block:

```rust
    let index = (validator_commitment % (validators.len() - 1))
    let proposing_validator = validators[index as usize]
```

For the remaining time of the current round *r*, the selected `proposing_validator` may propose a new Block and gossip it to other validator. Those validators will attest to the block by signing it and gossip it further. Once the signature threshold is met the Block will be stored by receiving nodes. 

The synchronization loop will help nodes that join the network catch up with Blocks they missed.

### Edge Case: Validator fails to commit

### Edge Case: Validator fails to propose

### Edge Case: Block has insufficient commitments e.o.r (End of Round)