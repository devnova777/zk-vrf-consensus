# Zero Knowledge Random Number Generator

The zero knowledge random number generator described in `02-PoRD-abstract.md` is quite straightforward and can be implemented in any ZKVM or domain specific language of choice. 

Random numbers used in PoRD could be generated without zero knowledge, but this is not advisible since the private random seed would get lost and therefore anyone could predict the random numbers produced by a node for a round knowing just the public key and the height (or whatever public parameters were chosen).

The objective when using ZK for this task is to obfuscate the random seed and verify that the random number is valid for a consensus round.

What's great about the zero knowledge random number is that during the commitment phase to the Proposals, no Node knows what `aR` will ultimately be. Therefore it is not possible to change the timestamp in such a way that the odds of winning the next round increase. If a node actively decides to commit an invalid timestamp then that is not necessarily a problem. An additional layer of security could be added by performing an outlier elimination over the timestamps in the commitments.

If we didn't use zk random numbers and instead just hashed the public key and nonce for each Node, then commitments would be manipulated to influence `aT` away from, or towards `aR`. 

Example: A malicious actor precomputes the random values for each live validator and determines that for the next Block height the value of `aR` will likely be larger than the average timestamp. Now an influencial actor could commit timestamps in such a way that either other Nodes' `aT` decreases or their own `aT` increases. The zk random number is only required to prevent this exact behavior.

Thanks to zero knowledge proofs, we can obfuscate the seed and make it invisible, yet preserve the verifiability and prove that the random number was generated for a Node's public key using the current height of the next Block as nonce.


