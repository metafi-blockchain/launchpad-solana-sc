

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;


use crate::{ AdminAccount, IdoAccount, AUTHORITY_ADMIN, AUTHORITY_IDO};


#[derive(Accounts)]
#[instruction(
    raise_token: String,
    rate: u16,
    open_timestamp: i64,
    allocation_duration: u32,
    fcfs_duration: u32,
    cap: u64,
    release_token: String,
    ido_id: u64)]
pub struct InitializeIdoAccount<'info> {
    #[account(init_if_needed,  
        payer = authority,  space = 8 + 2442,  
        seeds = [AUTHORITY_IDO , ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account:  Box<Account<'info, IdoAccount>>,
    #[account(init_if_needed,  payer = authority,  space = 8 + 65,  
        seeds = [AUTHORITY_ADMIN, ido_account.key().as_ref()], bump)]
    pub ido_admin_account:Box<Account<'info, AdminAccount>>,
    pub token_mint: Account<'info, Mint>,
    #[account(init_if_needed,  payer = authority, associated_token::mint = token_mint, associated_token::authority = ido_account)]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    // pub program_id: UncheckedAccount<'info>,
}
