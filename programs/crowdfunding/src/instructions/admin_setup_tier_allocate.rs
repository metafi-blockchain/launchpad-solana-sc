use anchor_lang::prelude::*;

use crate::{ AdminAccount, IdoAccount, AUTHORITY_ADMIN, AUTHORITY_IDO, AUTHORITY_USER, PdaUserStats};


#[derive(Accounts)]
#[instruction(
    index: u8,
    address: Pubkey,
    remove: bool)]
pub struct ModifyTierAllocatedOne<'info> {
    #[account( init_if_needed, payer = authority, space = 8+32+32+16+16+1+1, 
        seeds = [AUTHORITY_USER, ido_account.key().as_ref(), address.as_ref()], bump)]
    pub user_ido_account: Box<Account<'info, PdaUserStats>>,
    #[account(mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account( has_one = authority, 
        constraint = ido_account.key() == admin_wallet.owner, 
        constraint = authority.key() == admin_wallet.authority,
        seeds = [AUTHORITY_ADMIN, ido_account.key().as_ref()], bump)]
    pub admin_wallet: Box<Account<'info, AdminAccount>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
