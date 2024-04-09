# History and Motivation

Hi there, I am Jonas. I am a 23 year old Blockchain engineer working for a proof-of-stake Smart Contract Platform named "Casper". I have been interested in Consensus protocols for 7 years now and spent a lot of time implementing different components that make up Blockchain networks from scratch. Having hands-on experience with both POW and POS, I am aware that both protocols have their strenghts and weaknesses. One weakness that both have in common that bothers me personally is the high barrier of entry.

In 2019-2020 I decided to implement my own Blockchain in Python. Back then I didn't know nearly as much about Blockchain networks as I do today. Therefore I quickly developed a proof of concept that was flawed in many ways. My failed Python Blockchain experiment "Inpigritas", which i deprecated upon noticing that it lacks a consensus protocol, is open-source and can be cloned from Github @jonas089.

To not exceed the scope of this paper, I would like to summarize "Inpigritas" as a failed attempt of inventing a Blockchain where the next Block is predefined in the current Block e.g. Block n's Hash and Timestamp are declared in Block n-1. Today I know that this cannot work since the data that is included in the Block needs to affect the Block hash. Additionally, it must be decided which Block will be included in the Chain. With Inpigritas I made the mistake to think that it would be feasible for all nodes to create the same Blocks at the same time.

After prototyping my initial idea for Inpigritas, I found out that Intel used to experiment with a conesnsus protocol known as PoET "Proof of Elapsed Time", which sadly was not decentralized since it was highly dependant on Intel cloud infrastructure and in-house virtualization.

Now, In 2023, as a member of a core team working on zero knowledge scaling solutions for the Casper Blockchain, I have more hands-on experience and a broader horizon. This enabled me to develop PoDT to a point where I think that it can be implemented and potentially even used in either a decentralized sequencer, or even a full-node Blockchain Layer 1.
