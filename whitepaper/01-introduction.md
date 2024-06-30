# Introduction to Blockchain Consensus

Blockchain consensus is required for all decentralized networks where Blocks are generated according to any Blockchain protocol. A Blockchain protocol is a set of rules that determine under which conditions Blocks are created and how operating Nodes is economically incentivised.

The issue that is solved with consensus mechanisms is that of simultaneous transaction processing of different nodes in a Network. Nodes will synchronize Transactions as they receive them, but ultimately all transactions that occur between two Blocks need to be included in a single Block. Therefore one Node in the Network must be selected that is eligible of creating the next Block. 

# Introduction to Proof of Work: Bitcoin Consensus

The Bitcoin protocol states that which Node gets to create the next Block and receive a `Block Reward` is determined through a competing algorithm, where a cryptographic operation is performed repeatedly until a result that is valid for the current Block and lies within a certain range is produced. 

The Node that first computes such a valid result gets to create the next Block and claim the reward.

# Introduction to Proof of Stake: Casper, Ethereum Consensus

Proof of Stake protocols usually collect commitments from a set of Nodes eligible of creating Blocks to then randomly select the next Block creator. 

In order to become eligible of creating a new Block one has must delegate a certain Token threshold / `stake`.

# Introduction to Proof of Random Delta: A novel, zk-based consensus mechanism

With Proof of Random Delta a new consensus mechanism enters the scene that will be described in more detail in chapter `02-PoRD-abstract.md`. 

PoRD relies on verifiable random number generation and independent commitments to select the next node eligible of creating a Block. 

In PoRD every node can propose a valid Block, but similar to POS and POW only one Block will be chosen and included in the Blockchain for each round.

# Benefits of Proof of Random Delta 

PoRD offers several benefits compared to other consensus mechanisms like POW and POS, but especially relevant is the low barrier of entry for new node operators. Since no expensive equipment or large token delegation is required, anyone can participate in consensus and run a node on cheap, standard hardware. This protopery largely benefits the degree of decentralization that is theoretically achievable with PoRD.

| Property | POS | POW | PoRD |
|---|---|---|---|
| $$$ | y | y | n |
| economically friendly | y | n | y |
| fully decentralized | n | y | n |
| established | y | y | n |
| economic incentive for node operators | y | y | y |

PoRD is obviously not yet widely criticized since it has not been used in a production Blockchain, the poor reputation of POS and POW however are yet another reason why one may choose a novel, zk-based consensus mechanism.
