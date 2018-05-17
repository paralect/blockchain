# blockchain
A blockchain and smart contracts research project

### For easy access wiki is located here in README.md temporarily.
-----------------------------------------

Welcome to the blockchain wiki!

**Table of Contents:**

0. [References & Good Reads](#0)
1. [Blockchain](#1)  
   a. [What is Blockchain?](#1a)  
   b. [Block Structure](#1b)  
   c. [How Blockchain works](#1c)    
2. [Comparison of smart-contract platforms](#2)
3. [Selected smart-contract platform](#3)
4. [Ideas for smart contacts](#4)  
   a. [Simple Voting DApp with Ethereum and React (Updated)](#4a)  
   b. [Token Sale (ICO) Website](#4b)  
   c. [Crowdsale (ICO) DApp (New)](#4c)  
   d. [EOS Wallet or Explorer/IDE (New)](#4d)
5. [To Discuss](#5)
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

<a name="1a"></a>  
### **a) What is Blockchain:**  
(short)  
A blockchain is a continuously growing list of records, called blocks, which are linked and secured using cryptography. 

(longer)  
It is "an open, distributed ledger that can record transactions between two parties efficiently and in a verifiable and permanent way". For use as a distributed ledger, a blockchain is typically managed by a peer-to-peer network collectively adhering to a protocol for validating new blocks. Once recorded, the data in any given block cannot be altered retroactively without the alteration of all subsequent blocks, which requires collusion of the network majority.

Source: https://en.wikipedia.org/wiki/Blockchain

<a name="1b"></a>  

### **b) Block structure:**  

**i. Basic block structure**

<img src="https://github.com/paralect/blockchain/blob/master/wiki/images/blockhain-block.
" width="900" />   
 
Source: https://medium.com/@lhartikk/a-blockchain-in-200-lines-of-code-963cc1cc0e54  

**ii. Block structure in Bitcoin blockchain**  

<img src="https://github.com/paralect/blockchain/blob/master/wiki/images/block_bitcoin.jpg" width="560" />   
 
Source: http://computersecuritypgp.blogspot.com/2016/05/what-is-blockchain.html

<a name="1c"></a>  

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
| **Long Definition**    | Decentralized OS with cryptoeconomic incentive which can support industrial-scale decentralized applications                                                                                                                                                                                                                                                               | [1] Open software platform enables developers to build and deploy decentralized applications <br> [2] Ethereum is an open-source, public, blockchain-based distributed computing platform and operating system with smart contract functionality                                                                                                                                                          |                                                                                                                                                                                                                                                                                                                                                                 |
| **Key Features**       | - Completely remove transaction fees <br> - Conduct millions of transactions per second <br><br> - Designed to enable vertical and horizontal scaling of decentralized applications. (which is achieved by software providing accounts, authentication, databases, asynchronous communication and the scheduling of applications **across hundreds of CPU cores or clusters**.)                                                                                                                                                                                                                                                         | - Enterprise Ethereum Alliance (EEA)July 2017, there were over 150 members in the alliance <br><br> (including ConsenSys, CME Group, Cornell University's research group, Toyota Research Institute, Samsung SDS, Microsoft, Intel, J.P. Morgan, Cooley LLP, Merck KGaA, DTCC, Deloitte, Accenture, Banco Santander, BNY Mellon, ING, and National Bank of Canada, MasterCard, Cisco Systems, and Scotiabank) |
| **Blockchain Network** | - EOS creators block.one will not be launching a public eos blockchain. <br> - Instead leave it up to the community to do what they will with the EOS.io software.                                                                                                                                                                                                         | Ethereum is a distributed public blockchain network                                                                                                                                                                                                                                                                                                                                                       |
| **Smart Contract**   |     | A contract in the sense of Solidity is a collection of code (its functions) and data (its state) that resides at a specific address on the Ethereum blockchain                                                                                                                                                                                                                                                                                                                                          |       
| **Smart Contract Features**   | Contracts can be updated after published. <br><br> (Todo: Sounds good but isn’t contract conceptually an **immutable** block in blockchain ?)                                                                                                                                                                                                                                                                                                                                             |         The only possibility that code is removed from the blockchain is when a contract at that address performs the “selfdestruct” operation. <br><br> (The remaining Ether stored at that address is sent to a designated target and then the storage and code is removed from the state) <br> https://solidity.readthedocs.io/en/develop/introduction-to-smart-contracts.html#
| **Code Execution**   | EOS.IO based blockchains execute user-generated applications and code using WebAssembly (WASM). (WASM is an emerging web standard with widespread support of Google, Microsoft, Apple, and others. At the moment the most mature toolchain for building applications that compile to WASM is clang/llvm with their C/C++ compiler.)                                                                                                                                                                                                                                                                                                                                            | The code in Ethereum contracts is written in a low-level, stack-based bytecode language, referred to as "Ethereum virtual machine code" or "EVM code".  <br> https://github.com/ethereum/wiki/wiki/White-Paper#code-execution                                                                                                                                                                                                                                                                                                                                                                      |
| **Token/ Currency**    | EOS: <br> - Having EOS token gives the developer the right to use the some percentage of the whole systems resources. EOS token is never consumed. <br>  - EOS token holders will be able to rent / delegate their their share of resources to other developers                                                                                                                | Ether: <br> - Ethereum rents out their computational power to the developers. <br> - Miners work to earn Ether, a type of crypto token that fuels the network.                                                                                                                                                                                                                                                                                                                         |
| **Who is behind?**     | The core team behind EOS is “Block.one”, which is based in the Cayman Islands. <br>  Dan Larimer, is the CTO. He is the creator of delegated proof-of-stake and decentralized autonomous organizations aka DAOs. He is the also the man behind BitShares and Steem.                                                                                                        | Ethereum was proposed in late 2013 by Vitalik Buterin, a cryptocurrency researcher and programmer. Development was funded by an online crowdsale that took place between July and August 2014.[6] The system went live on 30 July 2015, with 11.9 million coins "premined" for the crowdsale. <br> The core Ethereum team was Vitalik Buterin, Mihai Alisie, Anthony Di Iorio, and Charles Hoskinson      |
| **Transaction Speeds** | Designed to perform millions of transactions per sec. <br> <br> Note: <br> Visa manages 24,000 transactions per second while Paypal manages 193 transactions per second. Compared to that, Bitcoin manages just 3-4 transactions per second while Ethereum fairs slightly better at. <br> https://howmuch.net/articles/crypto-transaction-speeds-compared <br> http://www.blocktivity.info/ | 20 transactions per sec                                                                                                                                                                                                                                                                                                                                                                                   |
|  **Consensus Algorithm**                  | DPOS aka the Delegated Proof of Stake consensus mechanism, they can easily compute millions of transactions per second.                                                                                                                                                                                                                                                    | Proof of Work (PoW) <br> (Plans to move to Proof of Stake (PoS) a new design called Casper)                                                                                                                                                                                                                                                                                                               |
| **Admin Tools**        |                                                                                                                                                                                                                                                                                                                                                                            | Mist: <br> UI digital wallet to trade & store Ether. And to write, manage, deploy and use smart contracts                                                                                                                                                                                                                                                                                            |                                                                                                                                                                                                                                                                                                                                                                          | Ethereum provides a decentralized Turing-complete virtual machine, the Ethereum Virtual Machine (EVM), which can execute scripts using an international network of public nodes.                                                                                                                                                                                                                          |
| **Block Time**        |               3 seconds                                                                                                                                                                                                                                                                                                                                                              | 13 seconds (for comparison, Bitcoin 10 mins) <br> https://etherscan.io/chart/blocktime                                                                                                                                                                                                                                                                                                                    |
| **Gas**                |                                                                                                                                                                                                                                                                                                                                                                           | "Gas", an internal transaction pricing mechanism, is used to mitigate spam and allocate resources on the network                                                                                                                                                                                                                                                                                          |

---

<a name="3"></a>  
## 3. Selected smart-contract platform 

**Assuming (!)** EOS will implement all the features they claim (ETA: June, 2018), EOS almost certainly seems to be right platform. 

However, since EOS is still under development, and although Ethereum has some unacceptible limitations, currently Ethereum can be a starting point for smart contract development since they are the only live blockchain network with working smart contracts. 

<a name="4"></a>  
## 4. Ideas for smart contacts 
<a name="4a"></a>  
### 4.a. Simple Voting DApp with Ethereum and React  

Conceptually, subject of voting seems to be a perfect fit for blockchain.

<h1>
  <img src="https://raw.githubusercontent.com/simsekgokhan/Voting-DApp-Ethereum/master/src/screenshot.jpg" width="900">  
</h1>

More: https://github.com/simsekgokhan/Voting-DApp-Ethereum

<a name="4b"></a>  
### 4.b. Token Sale (ICO) Website

Raw information from emails:  

""   
Dmitry Schetnikovich:  
  I had a talk with Alex Shkor recently and he proposed an additional idea to consider: Token Sale (or ICO) website in Etherium or EOS networks. There are a lot of examples and materials about this process. For instance: [Token Sale Smart Contracts](https://blog.bluzelle.com/token-sale-smart-contracts-6ab03f7cda2a) ([GitHub](https://github.com/bluzelle/ico-smart-contracts)), [EOS ICO Step by Step guide](https://steemit.com/eos/@nadejde/eos-ico-step-by-step-guide-beginner) etc.

People initiate ICO (Token Sale) even before they complete implementation of their application (or "protocol"). This is a kind of a service that Paralect can provide. According to Alex, process of implementation of Token Sale Website is straightforward, after you did it for the first time. 

Token Sale / ICO Website (simplified):

1) Presents an idea how we are going to change the world (we skip this step, because this content will be provided by the client)
2) Explains how this ICO is going to work and what are the rules (together with client we need to find this rules)
3) Has a button "Buy this tokens".
4) Allows you to exchange your current digital cons or tokens (ETH, BTC, etc.) for the new one. 

It makes sense to implement it in both networks: for Etherium and EOS. Although Alex thinks, that in a couple of months EOS will go public and there will be a lot of demand for "EOS Token Sale / ICO" website. 

"" 

<a name="4c"></a>  
### 4.c. Simple Crowdsale (ICO) Dapp (Crowdsale and Custom Token contracts)  

This Crowdsale dapp consists of two contracts: Crowdsale (ICO) and Token contracts.  

- [master/ethereum/crowdsale/with_simple_fixed_token/Crowdsale.sol](https://github.com/paralect/blockchain/blob/master/ethereum/crowdsale/with_simple_fixed_token/Crowdsale.sol)  
- [master/ethereum/crowdsale/with_simple_fixed_token/TokenERC20.sol](https://github.com/paralect/blockchain/blob/master/ethereum/crowdsale/with_simple_fixed_token/TokenERC20.sol)  

For more advanced version of this dapp, see below:  

- [master/ethereum/crowdsale/with_adv_token/Crowdsale-2.sol](https://github.com/paralect/blockchain/blob/master/ethereum/crowdsale/with_adv_token/Crowdsale-2.sol)  
- [master/ethereum/crowdsale/with_adv_token/MyAdvancedToken.sol](https://github.com/paralect/blockchain/blob/master/ethereum/crowdsale/with_adv_token/MyAdvancedToken.sol)  


**Note:** 
For this dapp, modified versions of official Ethereum Crowdsale and Token templates in the links below are used.  

- https://ethereum.org/crowdsale  
- https://www.ethereum.org/token

**How this Dapp works:**  

<img src="https://github.com/paralect/blockchain/blob/master/ethereum/crowdsale/summary_diagram_of_crowdsale_with_simple_fixed_token.png" width="900" />    

<a name="4d"></a>  
### 4.d. EOS Wallet or Explorer/IDE

Ethereum Wallet or Bitcoin Core like desktop app for EOS blockchain. 

**Possible Features:** 
- Account explorer / management  
- Contract method executions  
- Contract IDE/compilation (using eosio executables)   
- Contract deployment 
- Block explorer  
- Transaction explorer  

**E.g.: Screenshot from Ethereum Wallet: Contract deployment**

<img src="https://github.com/paralect/blockchain/blob/master/ideas/EOS_wallet_explorer_IDE/Ethereum-Wallet-Screenshot.png" width="900" />    



<a name="5"></a>  

## **5. To Discuss:**  

**// eth and btc scalability problem**  
https://github.com/ethereum/wiki/wiki/White-Paper#scalability  

The problem with such a large blockchain size is centralization risk. 
If the blockchain size increases to, say, 100 TB, then the likely scenario would be 
that only a very small number of large businesses would run full nodes, with all 
regular users using light SPV nodes. In such a situation, there arises the potential 
concern that the full nodes could band together and all agree to cheat in some 
profitable fashion (eg. change the block reward, give themselves BTC).

**// problems of decentralized applications**  
Despite bringing a number of benefits, decentralized applications aren't faultless. 
Because smart contract code is written by humans, smart contracts are only as good 
as the people who write them. Code bugs or oversights can lead to unintended adverse 
actions being taken. If a mistake in the code gets exploited, there is no efficient way 
in which an attack or exploitation can be stopped other than obtaining a network consensus 
and rewriting the underlying code. This goes against the essence of the blockchain which 
is meant to be immutable. Also, any action taken by a central party raises serious 
questions about the decentralized nature of an application.  

**// dapps projects currently in development on Ethereum**   
https://www.stateofthedapps.com/  

**// Decentralized Autonomous Organizations (DAO)**  

Ethereum can also be used to build Decentralized Autonomous Organizations (DAO). 
A DAO is fully autonomous, decentralized organization with no single leader. 
DAO’s are run by programming code, on a collection of smart contracts written on the 
Ethereum blockchain. 

The code is designed to replace the rules and structure of a traditional organization, 
eliminating the need for people and centralized control. 
A DAO is owned by everyone who purchases tokens, but 
instead of each token equating to equity shares & ownership, tokens act as contributions 
that give people voting rights.

**// btc Mining Centralization problem**  
https://github.com/ethereum/wiki/wiki/White-Paper#mining-centralization  

Mining algorithm is vulnerable to two forms of centralization.  

First, the mining ecosystem has come to be dominated by ASICs 
(application-specific integrated circuits), computer chips designed for, and therefore 
thousands of times more efficient at, the specific task of Bitcoin mining. 
This means that Bitcoin mining is no longer a highly decentralized and egalitarian pursuit, 
requiring millions of dollars of capital to effectively participate in.

Second, most Bitcoin miners do not actually perform block validation locally; instead, 
they rely on a centralized mining pool to provide the block headers. This problem is 
arguably worse: as of the time of this writing, the top three mining pools indirectly 
control roughly 50% of processing power in the Bitcoin network, although this is mitigated 
by the fact that miners can switch to other mining pools if a pool or coalition attempts a 
51% attack.

**// eth DAO (Decentralized Autonomous Organizations)**  
https://github.com/ethereum/wiki/wiki/White-Paper#decentralized-autonomous-organizations  

The general concept of a "decentralized autonomous organization" is that of a virtual entity 
that has a certain set of members or shareholders which, perhaps with a 67% majority, 
have the right to spend the entity's funds and modify its code. The members would collectively 
decide on how the organization should allocate its funds.

how to code a DAO is as follows. 
The simplest design is simply a piece of self-modifying code that changes if two thirds 
of members agree on a change. Although code is theoretically immutable, one can easily 
get around this and have de-facto mutability by having chunks of the code in separate 
contracts, and having the address of which contracts to call stored in the modifiable 
storage. 


...
