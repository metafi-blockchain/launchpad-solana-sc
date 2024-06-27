

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;


use crate::{ AuthRole, AuthorityRole, IDOProgramErrors, IdoAccount, InitializeIdoParam, OnePad, AUTHORITY_IDO, ONEPAD, OPERATOR_ROLE};


#[derive(Accounts)]
#[instruction(
   param: InitializeIdoParam
)]
pub struct InitializeIdoAccount<'info> {

    #[account(
        seeds = [ONEPAD],
        bump = onepad_pda.bump,
        constraint = onepad_pda.has_operator(operator_pda.key())@ IDOProgramErrors::OnlyOperatorAllowed,
    )]
    pub onepad_pda: Box<Account<'info, OnePad>>,
    #[account(init,  
        payer = authority,  space = 8 + 2442,  
        seeds = [AUTHORITY_IDO , param.ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_pda.bump,
        constraint = operator_pda.has_authority(authority.key(), AuthRole::Operator ) == true @ IDOProgramErrors::OnlyOperatorAllowed,
    )]
    pub operator_pda: Account<'info, AuthorityRole>,
    pub token_mint: Account<'info, Mint>,
    #[account(init_if_needed,  payer = authority, associated_token::mint = token_mint, associated_token::authority = ido_account)]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


pub fn handle_initialize_ido(
    ctx: Context<InitializeIdoAccount>,
    params: InitializeIdoParam
) -> Result<()> {

    let ido_account = &mut ctx.accounts.ido_account;
    let token_mint = &ctx.accounts.token_mint;
    let operator_pda = &ctx.accounts.operator_pda;
    
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
        &operator_pda.key(),
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
    msg!("Create ido success!");
    Ok(())
}



#[derive(Accounts)]
#[instruction(
   param: InitializeIdoParam
)]
pub struct InitializeIdoNative<'info> {

    #[account(
        seeds = [ONEPAD],
        bump = onepad_pda.bump,
        constraint = onepad_pda.has_operator(operator_pda.key())@ IDOProgramErrors::OnlyOperatorAllowed,
    )]
    pub onepad_pda: Box<Account<'info, OnePad>>,
    #[account(init,  
        payer = authority,  space = 8 + 2442,  
        seeds = [AUTHORITY_IDO , param.ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_pda.bump,
        constraint = operator_pda.has_authority(authority.key(), AuthRole::Operator ) == true @ IDOProgramErrors::OnlyOperatorAllowed,
    )]
    pub operator_pda: Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handle_initialize_ido_native(
    ctx: Context<InitializeIdoNative>,
    params: InitializeIdoParam
) -> Result<()> {

    let ido_account = &mut ctx.accounts.ido_account;
    
    let operator_pda = &ctx.accounts.operator_pda;
    
    let InitializeIdoParam {
        raise_token,
        rate,
        open_timestamp,
        allocation_duration,
        fcfs_duration,
        cap,
        ido_id,
    } = params;

    let decimal = 9;
    ido_account.create_ido(
        &operator_pda.key(),
        &raise_token,
        &decimal,
        &rate,
        &open_timestamp,
        &allocation_duration,
        &fcfs_duration,
        &cap,
        &ido_id,
        &ctx.bumps.ido_account,
    )?;
    Ok(())
}