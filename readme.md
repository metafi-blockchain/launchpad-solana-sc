# Test smart contract 


1. Install rust
2. Install cargo
3. Install nodejs




require:
rustc 1.75.0
solana-cli 1.18.1
anchor-cli 0.29.0


run command:

anchor deploy    //deploy code to blockchain
anchor test  //run deploy and test
anchor build --verifiable       //verify code

solana rent 1024
check log: ❯ solana confirm -v 32EXWwSFerpucapNyEC4XvsRyJY1SqfqSmUxBvLQKUFKfeatCYEeydEQun7cQvTtNS44iSKZaAFaH6UNpesNwxGk