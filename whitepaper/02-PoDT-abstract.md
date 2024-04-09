# PoDT Abstract

This abstract explains how PoDT can be implemented and how it works with zero knowledge proofs in mind.

## Block Proposal and Commitments

Before a new Block can be created, a selection process must happen where a single Node is chosen that is eligible of creating the next Block. In between Blocks Nodes synchronize transaction data with live peers.

During the selection process every Node that wants to participate in consensus will sign a Proposal.

A proposal may look like this:

```rust
struct Proposal{
    signature: Signature,
    timetamp: Timestamp,
    nonce: BlockHeight,
    commitments: Vec<Commitment>
}
```

Every other peer must commit to each proposal. A commitment may look like this:

```rust
pub struct Commitment{
    signature: Signature
    timestamp: Timestamp,
}
```

When a Proposal is considered valid / how many commitments are required for it to be considered valid will be discussed later.

For all valid proposals the mean of the timestamps in the commitments will be evaluated and denoted as `aT`.

Next, a zero knowledge random number generator is used, that takes as public input the public key of a validator, as well as the height of the current Block. The private input to the random number generator will be a random seed chosen by the validator.

All participating Nodes will commit a random value to a vector for the current height, denoted as Vec<`cR`>. The zero knowledge random number generator is used so that it can be verified that the random number was generated with valid public parameters that match the commitment signature of the Node.

```rust
// abstract zk random number generator
let inputs = env::read();
let pub_in = inputs.public;
let priv_in = inputs.private;
env::commit(sha256(pub_in , priv_in))
```

## Choosing a Winner

The mean value of the `cR` vector will be calculated and denoted as `aR`. The Node that owns the Proposal with a mean commitment timestamp `aT` that is closest to `aR` will be chosen by the consensus to create the next Block. All other Nodes will not be eligible of creating this Block for the current session.

## Resolving Conflicts and Collisions in aT

In cases where `aT` of multiple Proposals collide, the zk random number generator is used repeatedly until a single winner is determined.

During the conflict resolution process a random value is created for each colliding `aT` and added to the original `aT`. The result will be the new `aT` for that Proposal and the consensus protocol repeats until all conflicts were resolved.

## Valid Proposals

The protocol must enforce a rule that states how many commitments e.g. what % of the total active peers are required for a single Proposal. This is a critical aspect of the consensus protocol and it introduces a problem.

A low barrier of entry in PoDT makes it a potential target for 51% attacks. An ideal scenario would require all active Nodes to commit to each Proposal.

This is however not always realistic since downtimes need to be considered and the decision making process must be repeated in cases where a single Node refuses or fails to contribute to a commitment round.

There are several options to tackle this challenge and successfully implement PoDT:

- A Majority threshold can be employed, but it must be ensured that the threshold is high enough for a sufficient number of independent entities to be involved.

- Numerous trusted, independent entities could be chosen to operate a limited number of Nodes. An organization could onboard validators through KYC or other real-world processes.

So long as a single, or a small number of entities can't reach the majority threshold, the risk of a 51% attack is manageable.
