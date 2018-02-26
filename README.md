# blockchain
A blockchain and smart contracts research project

### For easy access wiki is located here in README.md temporarily.
-----------------------------------------

Welcome to the blockchain wiki!

**Table of Contents:**

0. [Sources & Good Reads](#0)
1. [What is Blockchain?](#1)
2. [Comparison of smart-contract platforms](#2)
3. [Selected smart-contract platform](#3)
4. [Ideas for smart contacts](#4) 
5. [...](#5)
-----------

<a name="0"></a>  
## 0. References & Good Reads

**Blockchain**    
Glossary: https://github.com/ethereum/wiki/wiki/Glossary   
Blockchain at Berkeley Uni.: https://www.youtube.com/channel/UC5sgoRfoSp3jeX4DEqKLwKg/playlists


**Bitcoin**  
Satoshi Nakamoto's original paper: https://bitcoin.org/bitcoin.pdf  

**Ethereum**  
Ethereum Overview : https://blockgeeks.com/guides/ethereum/  
Wiki: https://github.com/ethereum/wiki/wiki  
White-Paper: https://github.com/ethereum/wiki/wiki/White-Paper  
Design-Rationale: https://github.com/ethereum/wiki/wiki/Design-Rationale  
Yellow Paper: https://ethereum.github.io/yellowpaper/paper.pdf  
Decentralized-apps-(dapps): https://github.com/ethereum/wiki/wiki/Decentralized-apps-(dapps)  

**EOS**  
EOS Overview: https://blockgeeks.com/guides/eos-blockchain  
Wiki: https://github.com/EOSIO/eos/wiki  
White Paper: https://github.com/EOSIO/Documentation/blob/master/TechnicalWhitePaper.md


**Smart Contracts**  
EOS Smart Contracts: https://github.com/EOSIO/eos/wiki/Smart%20Contract  
Ethereum Smart Contracts: https://solidity.readthedocs.io/en/develop/introduction-to-smart-contracts.html#  
Video: [Ethereum - How to Create and Publish a Smart Contract](https://www.youtube.com/watch?v=TC-bDQZbXd0&list=PLcpJWTGPhadatTqKVM7_Ra_HhFtZ0SBi8&index=3)

<a name="1"></a>  
## 1. What is Blockchain?

### **a) What is Blockchain:**  
(short)  
A blockchain is a continuously growing list of records, called blocks, which are linked and secured using cryptography. 

(longer)  
It is "an open, distributed ledger that can record transactions between two parties efficiently and in a verifiable and permanent way". For use as a distributed ledger, a blockchain is typically managed by a peer-to-peer network collectively adhering to a protocol for validating new blocks. Once recorded, the data in any given block cannot be altered retroactively without the alteration of all subsequent blocks, which requires collusion of the network majority.

Source: https://en.wikipedia.org/wiki/Blockchain

### **b) Block structure:**  

**i. Basic block structure**

<img src="https://github.com/paralect/blockchain/blob/master/wiki/images/blockhain-block.png" width="900" />   
 
Source: https://medium.com/@lhartikk/a-blockchain-in-200-lines-of-code-963cc1cc0e54  


**ii. Block structure in Bitcoin blockchain**  

<img src="https://github.com/paralect/blockchain/blob/master/wiki/images/block_bitcoin.jpg" width="560" />   
 
Source: http://computersecuritypgp.blogspot.com/2016/05/what-is-blockchain.html


### **c) How Blockchain works:**

**Simple**  
<img src="https://github.com/paralect/blockchain/blob/master/wiki/images/how-blockchain-works-simple.png" width="700" />    

Source: https://bitsapphire.com/wp-content/uploads/2017/01/Blockchain-industry-innovation-or-overhyped.png

**In Detail:**

![](https://github.com/paralect/blockchain/blob/master/wiki/images/how-blockchain-works-detail.jpg)

Source: http://www.relativelyinteresting.com/wp-content/uploads/2016/06/how-a-bitcoin-transaction-works.jpg

----

<a name="2"></a>  
## 2. Comparison of smart-contract platforms (Draft)


|                    | EOS                                                                                                                                                                                                                                                                                                                                                                        | Ethereum                                                                                                                                                                                                                                                                                                                                                                                                  |
|--------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Short Definition**   | Decentralized operating system                                                                                                                                                                                                                                                                                                                                             | Decentralized operating system                                                                                                                                                                                                                                                                                                                                                                            |
| **Long Definition**    | Decentralized OS with cryptoeconomic incentive which can support industrial-scale decentralized applications                                                                                                                                                                                                                                                               | [1] Open software platform enables developers to build and deploy decentralized applications <br> [2] Ethereum is an open-source, public, blockchain-based distributed computing platform and operating system with smart contract functionality                                                                                                                                                          |
| **Key Features**       | - Completely remove transaction fees <br> - Conduct millions of transactions per second                                                                                                                                                                                                                                                                                    | - Enterprise Ethereum Alliance (EEA)July 2017, there were over 150 members in the alliance <br> (including ConsenSys, CME Group, Cornell University's research group, Toyota Research Institute, Samsung SDS, Microsoft, Intel, J.P. Morgan, Cooley LLP, Merck KGaA, DTCC, Deloitte, Accenture, Banco Santander, BNY Mellon, ING, and National Bank of Canada, MasterCard, Cisco Systems, and Scotiabank) |
| **Blockchain Network** | - EOS creators block.one will not be launching a public eos blockchain. <br> - Instead leave it up to the community to do what they will with the EOS.io software.                                                                                                                                                                                                         | Ethereum is a distributed public blockchain network                                                                                                                                                                                                                                                                                                                                                       |
| **Token/ Currency**    | EOS: <br> - Having EOS token gives the developer the right to use the some percentage of the whole systems power. EOS token is never consumed. <br>  - EOS token holders will be able to rent / delegate their their share of resources to other developers                                                                                                                | Ether:  Miners work to earn Ether, a type of crypto token that fuels the network.                                                                                                                                                                                                                                                                                                                         |
| **Who is behind?**     | The core team behind EOS is “Block.one”, which is based in the Cayman Islands. <br>  Dan Larimer, is the CTO. He is the creator of delegated proof-of-stake and decentralized autonomous organizations aka DAOs. He is the also the man behind BitShares and Steem.                                                                                                        | Ethereum was proposed in late 2013 by Vitalik Buterin, a cryptocurrency researcher and programmer. Development was funded by an online crowdsale that took place between July and August 2014.[6] The system went live on 30 July 2015, with 11.9 million coins "premined" for the crowdsale. <br> The core Ethereum team was Vitalik Buterin, Mihai Alisie, Anthony Di Iorio, and Charles Hoskinson      |
| **Transaction Speeds** | Designed to perform millions of transactions per sec. <br> <br> Note: <br> Visa manages 24,000 transactions per second while Paypal manages 193 transactions per second. Compared to that, Bitcoin manages just 3-4 transactions per second while Ethereum fairs slightly better at. <br> https://howmuch.net/articles/crypto-transaction-speeds-compared <br> http://www.blocktivity.info/ | 20 transactions per sec                                                                                                                                                                                                                                                                                                                                                                                   |
|  **Consensus Algorithm**                  | DPOS aka the Delegated Proof of Stake consensus mechanism, they can easily compute millions of transactions per second.                                                                                                                                                                                                                                                    | Proof of Work (PoW) <br> (Plans to move to Proof of Stake (PoS) a new design called Casper)                                                                                                                                                                                                                                                                                                               |
| **Admin Tools**        |                                                                                                                                                                                                                                                                                                                                                                            | Mist: <br> UI digital wallet to trade & store Ether. And to write, manage, deploy and use smart contracts                                                                                                                                                                                                                                                                                            |                                                                                                                                                                                                                                                                                                                                                                          | Ethereum provides a decentralized Turing-complete virtual machine, the Ethereum Virtual Machine (EVM), which can execute scripts using an international network of public nodes.                                                                                                                                                                                                                          |
| **Block Time**        |                                                                                                                                                                                                                                                                                                                                                                            | 13 seconds (for comparison, Bitcoin 10 mins) <br> https://etherscan.io/chart/blocktime                                                                                                                                                                                                                                                                                                                    |
| **Gas**                |                                                                                                                                                                                                                                                                                                                                                                            | "Gas", an internal transaction pricing mechanism, is used to mitigate spam and allocate resources on the network                                                                                                                                                                                                                                                                                          |

---

<a name="3"></a>  
## 3. Selected smart-contract platform 
...
