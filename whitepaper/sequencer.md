# Litepaper: Distributed Sequencer with ZK Consensus
Design and Implementation by [Jonas Pauli](https://www.linkedin.com/in/jonas-pauli/)

## Introduction
This paper describes my implementation of a distributed sequencer with a zero knowledge consensus protocol. Decentralized sequencing is difficult to achieve and usually involves multiple consensus layers that are reliant on complex tokenomics, or proof of work. This project strives to offer a simplified alternative to centralized sequencing with a fixed set of distributed validators rather than achieve the highest possible degree of decentralization. By design this sequencer has *no tokenomic model* but in theory one could be built on top of the simple consensus layer. The incentive behind this project is to offer a sequencer that is a distributed alternative to a centralized one, yet not as complex and expensive to setup and maintain as a fully decentralized one that was built on a staking layer.

## Implementation details: Work in Progress
A *POC* (Proof of Concept) implementation of this sequencer can be found [here](https://github.com/jonas089/distributed-sequencer).
This repository includes a docker compose that helps quickly setup a network of *4* nodes. The docker image currently builds reliably on x86 architecture. Also due to ongoing work there might be new instability introduced. Check the *release* tags for semi-reliable prototype versions, the *master* branch is under active and heavy development and breaking changes can occur.

I will update the status below to reflect the *stability* of the codebase, not the *efficiency* of the code written:

```
Stability: 

LOW
*MEDIUM
HIGH, 

last updated: Friday, September 13
```

### Consensus
Validator nodes collect arbitrary transactions, or rather, arbitrary transaction hashes, and store them in a temporary mempool. In the first iteration of the *POC*, transaction pools are not synchronized between nodes. Instead, each node commits its pool when proposing a block. This is not ideal, but assuming a small set of validators, it can be sufficiently fast.

For each block height *h* a round *r* is defined as:

```rust
    let round = (get_current_time() - last_block_unix_timestamp)
        / ROUND_DURATION;
```

A block is stored when a round concludes successfully. There can be multiple rounds for a single block height in case the selected validator fails to commit a random number for that round.

If the *committing_validator* fails to commit a random number by the end of the round, a new validator will be selected (the next validator in the fixed set of validators). The validator chosen to commit a random number is selected based on the following criteria:

```rust
    let committing_validator = validators[round % (validators.len() - 1)]
```

The selected *committing_validator* is eligble of committing a verifiable random value (generated using a hash-based zero knowledge verifiable random function) to other nodes. Those nodes will then deterministically evaluate the validator who is eligible of creating the next block:

```rust
    let index = (validator_commitment % (validators.len() - 1))
    let proposing_validator = validators[index as usize]
```

For the remaining time of the current round *r*, the selected *proposing_validator* may propose a new block and gossip it to other validators. Those validators will attest to the block by signing it and gossip it further. Once the signature threshold is met the block will be stored by receiving nodes.

The synchronization loop will help nodes that join the network catch up with blocks they missed.

### Solving Finality: Experimental for MVP
Finality will be reached by keeping track of the block with the lowest hash and by only proceeding to gossip blocks with a hash lower than that.
This is implemented such that when a block proposal is received by a node, it is either dropped or signed depending on it being the lowest:

```rust
let early_revert: bool = match &state_lock.consensus_state.lowest_block {
    Some(v) => {
        if proposal.to_bytes() < v.clone() {
            state_lock.consensus_state.lowest_block = Some(proposal.to_bytes());
            false
        } else {
            true
        }
    }
    None => {
        state_lock.consensus_state.lowest_block = Some(proposal.to_bytes());
        false
    }
};
if early_revert {
    return error_response;
}
```

This might occasionally override a synchronized block. Synchronized blocks are also checked against those in storage to determine whether or not 
they are lower. Should a synchronized block be lower than the block in storage, then it will replace that block in storage. As per the current design finality occurs after *1 block*.

### Edge Case: Validator fails to commit
Should the *committing_validator* fail to commit by the end of the round:

- a new round will be started with the next validator in que as the *committing_validator*
- there will be a clearing period of *c* seconds during which no new commitments or blocks may be accepted

### Edge Case: Validator fails to propose
- Should the selected *proposing_validator* fail to propose a block for the round, a new round will start automatically and the next validator will be selected. 
- In-between rounds the in-memory consensus state is reset and the next validator is selected from the set based on the current round.

The current round for the next block is calculated using this formula:
```rust
    current_round = ((current_timestamp - last_block_timestamp) / ROUND_DURATION) + 1
```

See ./byzantine-fault.md for some additional background on the safety and liveness of this novel consensus protocol.