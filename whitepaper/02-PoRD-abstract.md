# PoRD Abstract
This abstract explains how PoRD can be implemented.

## Block Proposal and Commitments
Before a new Block can be created, a selection process must happen where a single Node is chosen that is eligible of creating the next Block. In between Blocks Nodes synchronize transaction data with live peers.

During the selection process every Node that wants to participate in consensus will sign a Proposal.

A proposal may look like this:

```rust
struct Proposal{
    signature: Signature,
    timetamp: Timestamp,
    block: Block,
    nonce: BlockHeight,
    commitments: Vec<Commitment>
}

```

Idea:

A proposal can be created once a Node's transaction pool for the next Block has reached a certain threshold. 

For a Proposal to be valid, it must meet a commitment threshold (2/3rds of the validator set). A commitment can look like this:

```rust
pub struct BlockCommitment{
    signature: Signature
    timestamp: Timestamp
}
```

Before a Block can be proposed the validator eligible of creating that Block is selected.
Each validator participating in the consensus round commits a zk random number as a consensus commitment. The mean of these zk random numbers is denoted as `aC`.

```rust
// abstract zk random number generator
let inputs = env::read();
let pub_in = inputs.public;
let priv_in = inputs.private;
env::commit(sha256(pub_in , priv_in))
```

## Choosing a Winner

*An implementation of this algorithm can be found [here](https://github.com/jonas089/PoRD-sequencer/blob/master/src%2Fconsensus%2Flogic.rs)*

The Node that owns the Commitment closest to `aC` will be chosen by the consensus to create the next Block. All other Nodes will not be eligible of creating this Block for the current session.

Every Node can verify this selection process by re-evaluating the contributions.

For a Block to be accepted it must have a sufficient number of contributions and the commitments must be valid. The selection process that incorporates the Chaos function must also be verified.

## Valid Proposals
As mentioned earlier Proposals must meet a commitment threshold of 2/3rds. This measure helps prevent chain-splits.

A low barrier of entry in PoRD makes it a potential target for 51% attacks. Therefore it is recommended that a fixed set of trusted validators is announced. New validators could be voted in to further decentralize the network, but setting up fully anonymous validator nodes will not be possible.
