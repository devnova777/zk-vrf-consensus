# Background
Read about Byzantine Fault Tolerance [here](https://medium.com/tronnetwork/an-introduction-to-pbft-consensus-algorithm-11cbd90aaec) and [here](https://bitcoin.stackexchange.com/questions/58907/byzantine-fault-tolerant-consensus-why-33-threshold).

Byzantine Fault Tolerance (BFT) describes that in order to guarantee liveness and trust in a system with *3f + 1* participants, at least *2f + 1* must be have honest. Therefore at most *f* can be dishonest. 

# Context
BFT does not apply here. So long as the majority threshold *t* for valid blocks is set to > 50% of the *fixed* validator set and less than *t* validators are dishonest, no subset of nodes can collect sufficienlty many attestations for a malicious block for it to be accepted by the honest nodes (the majority). Each node enforces a fully deterministic set of rules for each block and for a given period only accepts blocks that were proposed by the deterministically selected validator from the set.

The reason why we don't have to rely on BFT is that we assume a *fixed set of validators*, not an arbitrarily growing network of decentralized nodes. This is one of the key differences between this *distributed* sequencer and other *decentralized* sequencers.

# Conclusion
So long as the validator threshold *t* is set to >50% of the size of the *fixed validator set*, liveness and safety should be guaranteed.
