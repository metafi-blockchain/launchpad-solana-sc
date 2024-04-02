use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount};
use crate::{IdoAccount, AUTHORITY_IDO, AUTHORITY_USER, PdaUserStats, IDOProgramErrors};

#[derive(Accounts)]
pub struct Participate<'info> {
    #[account(mut, seeds = [AUTHORITY_IDO , ido_account.ido_id.to_le_bytes().as_ref()], bump = ido_account.bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,

    #[account(mut, 
        constraint = user_pda_account.allocated == true,
        constraint = user_pda_account.address == user.key(),
        seeds = [AUTHORITY_USER,ido_account.key().as_ref(), user.key().as_ref()], bump = user_pda_account.bump)]
    pub user_pda_account: Account<'info, PdaUserStats>,

    #[account(mut,
        constraint = user_token_account.owner == user.key() @IDOProgramErrors::UserTokenAccountNotMatch,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut,
        constraint = ido_account._raise_token == ido_token_account.mint,
        constraint = ido_token_account.owner == ido_account.key() @IDOProgramErrors::IDoTokenAccountNotMatch,
    )]
    pub ido_token_account: Account<'info, TokenAccount>,
    #[account(signer)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}