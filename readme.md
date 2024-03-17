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


 solana program extend A7HQd8NLQAj5DRxZUXS5vNkpUfDhnDRkHS8KhrP8eP1t 20240

 anchor upgrade target/deploy/crowdfunding.so --provider.cluster devnet--program-id 6KMVQWmTXpd36ryMi7i91yeLsgM6S4BiaTX3UczEkvqq


anchor test --skip-build --skip-deploy
