use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use borsh::BorshDeserialize;
use mpl_token_metadata::{
    instruction::create_metadata_accounts_v3, pda::find_metadata_account, ID as MetadataID,
};
    
    declare_id!("8LgkDyXcYXtX6h78eNRydLr2tkWHxduREnpb5TZdgE8b");
    
    #[program]
    pub mod token_with_metadata {
        use super::*;
    
        pub fn initialize(
            ctx: Context<InitializeMint>,
            uri: String,
            name: String,
            symbol: String,
        ) -> Result<()> {
            let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("mint").unwrap()]];
            let signer = [&seeds[..]];
    
            let account_info = vec![
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ];
    
            invoke_signed(
                &create_metadata_accounts_v3(
                    ctx.accounts.token_metadata_program.key(), // token metadata program
                    ctx.accounts.metadata.key(),               // metadata account PDA for mint
                    ctx.accounts.mint.key(),                   // mint account
                    ctx.accounts.mint.key(),                   // mint authority
                    ctx.accounts.user.key(),                   // payer for transaction
                    ctx.accounts.mint.key(),                   // update authority
                    name,                                      // name
                    symbol,                                    // symbol
                    uri,                                       // uri (offchain metadata)
                    None,                                      // (optional) creators
                    0,                                         // seller free basis points
                    true,                                      // (bool) update authority is signer
                    true,                                      // (bool) is mutable
                    None,                                      // (optional) collection
                    None,                                      // (optional) uses
                    None,                                      // (optional) collection details
                ),
                account_info.as_slice(),
                &signer,
            )?;
    
            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        authority: ctx.accounts.mint.to_account_info(),
                        to: ctx.accounts.token_account.to_account_info(),
                        mint: ctx.accounts.mint.to_account_info(),
                    },
                    &signer,
                ),
                1,
            )?;
    
            Ok(())
        }
    }
    
    #[derive(Accounts)]
    pub struct InitializeMint<'info> {
        #[account(
            init,
            seeds = [b"mint"],
            bump,
            payer = user,
            mint::decimals = 6,
            mint::authority = mint,
        )]
        pub mint: Account<'info, Mint>,
        /// CHECK: Using "address" constraint to validate metadata account address
        #[account(
            mut,
            address=find_metadata_account(&mint.key()).0
        )]
        pub metadata: UncheckedAccount<'info>,
        #[account(
            init_if_needed,
            payer = user,
            associated_token::mint = mint,
            associated_token::authority = user
        )]
        pub token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub token_program: Program<'info, Token>,
        pub token_metadata_program: Program<'info, TokenMetaData>,
        pub associated_token_program: Program<'info, AssociatedToken>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }
    
    #[derive(Clone)]
    pub struct TokenMetaData;
    impl anchor_lang::Id for TokenMetaData {
        fn id() -> Pubkey {
            MetadataID
        }
    }