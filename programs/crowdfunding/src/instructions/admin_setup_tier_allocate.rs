use anchor_lang::prelude::*;

use crate::{ AdminAccount, IdoAccount, AUTHORITY_ADMIN, AUTHORITY_IDO, AUTHORITY_USER, PdaUserStats};


#[derive(Accounts)]
#[instruction(
    index: u8,
    address: Pubkey,
    remove: bool)]
pub struct ModifyTierAllocatedOne<'info> {
    #[account( init_if_needed, payer = authority, space = 8+32+32+16+16+1+1+320, 
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

pub fn modify_tier_allocated_one(
    ctx: Context<ModifyTierAllocatedOne>,
    index: u8,
    address: Pubkey,
    remove: bool,
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    let user_pda = &mut ctx.accounts.user_ido_account;

    //get data user pda
    if user_pda.bump != 0 && user_pda.address == address{
        user_pda.update_allocate(&index,  &!remove);
        ido_account.update_allocate_count( &(index as usize),  &!remove)?;

    }else {
        if !remove{
            user_pda.init_user_pda(&index, &address, &ido_account.key(), &!remove, &ctx.bumps.user_ido_account)?;
            ido_account.update_allocate_count( &(index as usize),  &!remove)?;
        }
    }
    
    Ok(())
}