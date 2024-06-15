

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;


use crate::{ AdminAccount, IdoAccount, InitializeIdoParam, AUTHORITY_ADMIN, AUTHORITY_IDO};


#[derive(Accounts)]
#[instruction(
   param: InitializeIdoParam
)]
pub struct InitializeIdoAccount<'info> {
    #[account(init,  
        payer = authority,  space = 8 + 2442,  
        seeds = [AUTHORITY_IDO , param.ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account(init,  payer = authority,  space = 8 + 65,  
        seeds = [AUTHORITY_ADMIN, ido_account.key().as_ref()], bump)]
    pub ido_admin_account:Box<Account<'info, AdminAccount>>,
    pub token_mint: Account<'info, Mint>,
    #[account(init_if_needed,  payer = authority, associated_token::mint = token_mint, associated_token::authority = ido_account)]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    // pub program_id: UncheckedAccount<'info>,
}


pub fn handle_initialize(
    ctx: Context<InitializeIdoAccount>,
    params: InitializeIdoParam
) -> Result<()> {

    let ido_account = &mut ctx.accounts.ido_account;
    let ido_admin_account   = &mut ctx.accounts.ido_admin_account;
    let token_mint = &ctx.accounts.token_mint;
    ido_admin_account._init_admin_ido(ctx.accounts.authority.key, &ido_account.key(), &ctx.bumps.ido_admin_account)?;
    
    let InitializeIdoParam {
        raise_token,
        rate,
        open_timestamp,
        allocation_duration,
        fcfs_duration,
        cap,
        ido_id,
    } = params;

    ido_account.create_ido(
        &ido_admin_account.key(),
        &raise_token,
        &token_mint.decimals,
        &rate,
        &open_timestamp,
        &allocation_duration,
        &fcfs_duration,
        &cap,
        &ido_id,
        &ctx.bumps.ido_account,
    )?;
    msg!("Create account success!");
    Ok(())
}
