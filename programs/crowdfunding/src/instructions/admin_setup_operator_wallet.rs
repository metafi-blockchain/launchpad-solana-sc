use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, ChangeOperatorWalletEvent, IDOProgramErrors, OnePad};
use crate::{ADMIN_ROLE, ONEPAD};

#[derive(Accounts)]
#[instruction(new_operator_wallet: Pubkey)]
pub struct SetUpOperatorWallet<'info> {
    #[account(
        mut,
        seeds = [ONEPAD],
        bump = onepad_pda.bump,
        constraint = onepad_pda.has_admin(admin_pda.key()) @ IDOProgramErrors::OnlyAdminAllowed,
    )]
    pub onepad_pda: Box<Account<'info, OnePad>>,

    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_pda.bump,
        constraint = admin_pda.has_authority(authority.key(), AuthRole::Admin ) == true @ IDOProgramErrors::OnlyAdminAllowed,
        constraint = admin_pda.status == true @ IDOProgramErrors::OnlyAdminAllowed,
    )]
    pub admin_pda: Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_change_operator_wallet(
    ctx: Context<SetUpOperatorWallet>,
    new_operator_wallet: Pubkey,
) -> Result<()> {
    let onepad_pda = &mut ctx.accounts.onepad_pda;

    let auth = &ctx.accounts.authority;
    require_keys_neq!(
        onepad_pda.operator_wallet,
        new_operator_wallet,
        IDOProgramErrors::OperatorWalletSameAsNewWallet
    );
    require_keys_neq!( new_operator_wallet, Pubkey::default(),IDOProgramErrors::AddressZero);
    onepad_pda.change_operator_wallet(new_operator_wallet)?;

    //emit event
    emit!(ChangeOperatorWalletEvent {
        operator_wallet: new_operator_wallet,
        time: Clock::get()?.unix_timestamp,
        admin: auth.key(),
    });
    Ok(())
}
