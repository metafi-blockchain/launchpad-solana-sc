use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;

use crate::{  AuthRole, AuthorityRole, IDOProgramErrors, IdoAccount, AUTHORITY_IDO, OPERATOR_ROLE};

#[derive(Accounts)]
pub struct SetupReleaseToken<'info> {
    #[account(mut,
        constraint = ido_account.authority == operator_pda.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump = ido_account.bump)]
    pub ido_account:  Box<Account<'info, IdoAccount>>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_pda.bump,
        constraint = operator_pda.has_authority(authority.key(), AuthRole::Operator ) == true @ IDOProgramErrors::OnlyOperatorAllowed,
    )]
    pub operator_pda: Account<'info, AuthorityRole>,
    #[account(init_if_needed,  payer = authority, associated_token::mint = token_mint, associated_token::authority = ido_account)]
    pub release_token_account: Account<'info, TokenAccount>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
pub fn setup_release_token(
    ctx: Context<SetupReleaseToken>,
    release_token: Pubkey,
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    let token_mint: &Account<'_, Mint> = &ctx.accounts.token_mint;
    // let token_pubkey = &Pubkey::from_str(&token).unwrap();
    // let pair_pubkey = &Pubkey::from_str(&pair).unwrap();
    let decimals = token_mint.decimals;
    ido_account.set_release_token(
        &release_token,
        &decimals,
    )?;

    Ok(())
}