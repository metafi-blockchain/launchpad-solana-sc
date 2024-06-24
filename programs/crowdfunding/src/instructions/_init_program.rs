
use anchor_lang::prelude::*;

use crate::{ AuthorityRole, OnePad, AuthRole};

use crate::{ ADMIN_ROLE, ONEPAD};

#[derive(Accounts)]


pub struct CreateOnePad<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 32*3 + 32*3 + 8,
        seeds = [ONEPAD],
        bump,
    )]
    pub onepad_pda: Account<'info, OnePad>,
    #[account(
        init,
        payer = authority,
        space = 60,
        seeds = [ADMIN_ROLE, authority.key().as_ref() ],
        bump,
    )]
    pub admin_role_pda: Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handle_initialize_onepad(ctx: Context<CreateOnePad>, operator_wallet: Pubkey) -> Result<()> {
    let onepad_pda = &mut ctx.accounts.onepad_pda;
    let admin_pda = &mut ctx.accounts.admin_role_pda;
    let authority = &ctx.accounts.authority;
    onepad_pda.initialize(&admin_pda.key(), &operator_wallet, ctx.bumps.onepad_pda)?;
    admin_pda.initialize(authority.key, ctx.bumps.admin_role_pda, AuthRole::Admin)?;
    Ok(())
}