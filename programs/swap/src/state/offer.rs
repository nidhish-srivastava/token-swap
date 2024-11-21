use anchor_lang::prelude::*;

/*
Token minting refers to the process of creating new tokens on a blockchain. In the context of blockchain networks like Solana, Ethereum, and others, "minting" is used to describe the creation of a new instance of a token (such as a cryptocurrency or utility token) that is added to a blockchain's ledger.

The minted tokens are issued to the creator's or a designated account's wallet and are typically used as the initial supply of a token. This process ensures that the newly minted tokens are recognized by the blockchain and can be transferred, traded, or used within decentralized applications (DApps) or smart contracts.

How Token Minting Works
Smart Contract or Program Creation:

In order to mint tokens, a smart contract (on Ethereum) or a program (on Solana) must first be created. This contract defines the rules and properties of the token, such as:
The name of the token.
The symbol (e.g., "SOL" for Solana, "USDT" for Tether).
The decimal places (how divisible the token is, e.g., 18 decimals for Ethereum-based tokens).
The total supply of tokens (or how many tokens will ever be created).
Minting Tokens:

Once the token smart contract or program is deployed, minting occurs through the program’s functionality. Minting usually involves sending a transaction to the contract or program specifying the number of tokens to create.
The minted tokens are then assigned to a specific address (typically the address of the creator or a designated recipient). These tokens are stored in token accounts (also known as wallets in blockchain terms) and can be transferred, sold, or used in decentralized applications.
Minting Tokens on Solana:

On Solana, tokens are created using the SPL Token Program, which is Solana's equivalent of the ERC-20 standard on Ethereum.
A "mint" in the SPL Token system refers to the token's origin or the public key that represents the token itself. Every token created on Solana has a unique mint address, which is the identifier for that token.
Minting an SPL token on Solana involves creating a new "mint" account on the blockchain and then transferring the minted tokens to a specific user’s account.
Example: If you're creating a new token on Solana (such as "MyCoin"), you'd need to create the "MyCoin" mint. After creating the mint, you can mint new tokens (say, 1000 units) and send them to a wallet.

Minting and Supply Control:

Some tokens have fixed supply, meaning that once they are minted, no more tokens can be created.
Other tokens might have inflationary models, where tokens are minted periodically to incentivize certain behaviors, such as staking or governance participation.
In cases where there’s no fixed supply, tokens can be minted in stages or based on demand or predefined rules.

*/


#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}