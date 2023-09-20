# Shops program (smart contract)
This repository demonstrates how to create/deploy and use a simple program (smart contract) on the
Solana blockchain.  
This is an updated mirror. For latest changes see: https://github.com/simsekgokhan/shop-ratings  

Info: 
- For simplicity, a Shop can have max 3 ratings (`struct Shop { ratings: [u32; 3] }`)
- Each Shop object and its data (e.g. ratings) lives in a unique PDA (a program's data account) on Solana blockchain

Features:
- Add new rating for a shop
- Read all ratings of a shop
- Set first rating of a shop (for testing)

E.g.  
Let's have a quick look at shop1  

a) shop1 data lives in this PDA:  
https://solscan.io/account/4roTv8dUHJrybx5goVLvwmewKWgMzo5h4dHPM8EcjydM?cluster=devnet  
&nbsp;&nbsp; // see the latest tx logs for more understanding 
  
b) Transaction: Adding a new rating of 66 for shop1:  
Client side (user) logs:   
```
--- Shop name: shop1

3. Write to chain: Sending tx
> Quick read before write:
> Shop obj: ShopSchema { ratings: [42, 0, 0] }
--- add_rating result: Ok(())

4. Read from chain:
> Shop obj: ShopSchema { ratings: [42, 66, 0] }
```

Program logs (blockchain side)  
```
Program log
#1 Unknown program instruction
> Program log: --- account.data before: [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
> Program log: --- instruction 1: Add new rating
> Program log: --- account.data after: [42, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0]
> Program ET588YffkqKZCCirkxd1NaR11tXKmNUSGCAcgbTrvrMG  consumed 7787 of 200000 compute units
> Program returned success
```
See the full logs at the end of this page:   
https://solscan.io/tx/4rq1efdNaLnP1kAjCC1FqyBUXXLTcMzn65UL3WvpxR28si7PgKfWnv79Y1AGUvMJxUeVmpjRGrBPgENL9toTAKXh?cluster=devnet  


# Usage

E.g.
```
// Deploy program (smart contract)
cd program
cargo build-bpf
solana program deploy target/deploy/helloworld.so 
Program Id: Es5dTX5VbmPfE7NVBD6hozEC6M77NCHNVtZ1BGMdU7M6
Fee: 10000
TX:
https://explorer.solana.com/tx/5YabTWTQcj6do8GhDqcc3XFe3YhRMFQWZCf8amtjLMAf7e1qzPs8pJi55xYJ91fbzrvBBiwNAEMpWeciGw9UEknN?cluster=devnet

// Use program (smart contract)
cd client
usage: e.g. 
  cargo r ../program/target/deploy/helloworld-keypair.json r shop1
  cargo r ../program/target/deploy/helloworld-keypair.json w shop1
(w: write, r: read)

// Adding second rating of 66 (for now, this value is hardcoded in main.rs)
cargo r ../program/target/deploy/helloworld-keypair.json w shop1

1. Connected to remote solana node running version (1.16.6).

(1_474_400) lamports are required for this transaction.
User: 7GDXzkmtqNG2BZmesUyv2qrbRoovv71TApd1bWSsZAuc
Balance: 23.448148251 Sol (23_448_148_251 lamports)

2. Create account for program to read/write its data...
... not created, account may already exist 

Program: ET588YffkqKZCCirkxd1NaR11tXKmNUSGCAcgbTrvrMG
Program's data account to read/write: 4roTv8dUHJrybx5goVLvwmewKWgMzo5h4dHPM8EcjydM
(derived addr for a given user and program combination)

--- Shop name: shop1
--- Shop size: 12 Bytes
--- Shop obj: struct Shop { ratings: [u32; 3] }

3. Write to chain: Sending tx
> Quick read before write:
--- program derived account: [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
> Shop obj: ShopSchema { ratings: [42, 0, 0] }
--- add_rating result: Ok(())

4. Read from chain:
--- program derived account: [42, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0]
> Shop obj: ShopSchema { ratings: [42, 66, 0] }

End

```
