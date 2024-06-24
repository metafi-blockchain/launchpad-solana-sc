use anchor_lang::prelude::*;

use crate::{ types::SetupUserTierAllocationParam, AuthRole, AuthorityRole, IDOProgramErrors, IdoAccount, PdaUserStats, AUTHORITY_IDO, AUTHORITY_USER, OPERATOR_ROLE};


#[derive(Accounts)]
#[instruction(
    params: SetupUserTierAllocationParam)]
pub struct ModifyTierAllocatedOne<'info> {
    #[account( init_if_needed, payer = authority, space = 8 + 61, 
        seeds = [AUTHORITY_USER, ido_account.key().as_ref(), params.address.as_ref()], bump)]
    pub user_ido_account: Box<Account<'info, PdaUserStats>>,
    #[account(mut,
        constraint = ido_account.authority == operator_pda.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump)]
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

pub fn handle_modify_tier_allocated(
    ctx: Context<ModifyTierAllocatedOne>,
    params: SetupUserTierAllocationParam) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    let user_pda = &mut ctx.accounts.user_ido_account;

    let SetupUserTierAllocationParam { tier,  address, remove,} = params;
    //get data user pda
    if user_pda.bump != 0 && user_pda.address == address{
        user_pda.update_allocate(&tier,  &!remove);
        ido_account.update_allocate_count( &(tier as usize),  &!remove)?;

    }else {
        if !remove{
            user_pda.init_user_pda(&tier, &address, &!remove, &ctx.bumps.user_ido_account)?;
            ido_account.update_allocate_count( &(tier as usize),  &!remove)?;
        }
    }
    
    Ok(())
}