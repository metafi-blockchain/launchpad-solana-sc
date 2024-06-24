
use anchor_lang::prelude::*;
use crate::{ AdminAccount, IdoAccount, SetAdminEvent, AUTHORITY_ADMIN, AUTHORITY_IDO};

#[derive(Accounts)]
pub struct UpdateAdminIdo<'info> {
    #[account(
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump = ido_account.bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account( mut,
        constraint = ido_account.key() == admin_wallet.owner,
        constraint = authority.key() == admin_wallet.authority,
        has_one = authority, seeds = [AUTHORITY_ADMIN, ido_account.key().as_ref()], bump = admin_wallet.bump)]
    pub admin_wallet: Account<'info, AdminAccount>,
    #[account(signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handle_update_admin_ido( ctx: Context<UpdateAdminIdo>, admin_address : Pubkey)->Result<()>{
    let admin_account = &mut ctx.accounts.admin_wallet;
    admin_account._set_admin(&admin_address)?;

    emit!(SetAdminEvent{
        admin_address: admin_address.to_string(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}