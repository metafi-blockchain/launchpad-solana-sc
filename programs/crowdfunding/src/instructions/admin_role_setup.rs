
use anchor_lang::prelude::*;

use crate::{ AdminAccount, IDOProgramErrors, IdoAccount, RoundClass, TierItem, AUTHORITY_ADMIN, AUTHORITY_IDO};


#[derive(Accounts)]
pub struct AdminModifier<'info> {
    #[account(
        mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump = ido_account.bump)]
    pub ido_account:Box<Account<'info, IdoAccount>>,
    #[account(
        mut,
        constraint = ido_account.key() == admin_wallet.owner,
        constraint = authority.key() == admin_wallet.authority,
        has_one = authority, seeds = [AUTHORITY_ADMIN, ido_account.key().as_ref()], 
        bump = admin_wallet.bump)]
    pub admin_wallet: Account<'info, AdminAccount>,
    #[account(signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn setup_releases(
    ctx: Context<AdminModifier>,
    from_timestamps: Vec<i64>,
    to_timestamps: Vec<i64>,
    percents: Vec<u16>,
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    //check size
    require!( from_timestamps.len() == to_timestamps.len(), IDOProgramErrors::InvalidReleaseIndex);
    require!( to_timestamps.len() == percents.len(),  IDOProgramErrors::InvalidReleaseIndex);

    ido_account.set_releases(
        &from_timestamps,
        &to_timestamps,
        &percents,
    )?;

    Ok(())
}

pub fn set_closed(ctx: Context<AdminModifier>, close: bool) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    ido_account.set_closed( &close)?;
    Ok(())
}

pub fn set_cap(ctx: Context<AdminModifier>, cap: u64) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    ido_account.set_cap(&cap)?;
    Ok(())
}

pub fn set_rate(ctx: Context<AdminModifier>, rate: u32) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    ido_account.set_rate( &rate)?;
    Ok(())
}
pub fn set_open_timestamp(ctx: Context<AdminModifier>, open_timestamp: i64) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    ido_account.set_open_timestamp( &open_timestamp)?;
    Ok(())
}

pub fn modify_tiers(ctx: Context<AdminModifier>, name_list: Vec<String>) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;

    require!(name_list.len() > 0, IDOProgramErrors::InValidTier);
    ido_account._tiers = vec![];
    //push tier into ido_account._tiers
    for (_, name) in name_list.iter().enumerate() {
        ido_account.add_tier(TierItem {
            name: name.to_string(),
            allocated_count: 0
        });
    }
    Ok(())
}

pub fn modify_tier(ctx: Context<AdminModifier>, index: u32, name: String) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;


    match ido_account._tiers.get_mut(index as usize) {
        Some(tier) => {
            tier.name = name;
        }
        None => {
            return err!(IDOProgramErrors::InvalidInDex);
        }
    }
    Ok(())
}

pub fn modify_round_allocations(
    ctx: Context<AdminModifier>,
    index: u8,
    tier_allocations: Vec<u64>,
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;

    match ido_account._rounds.get_mut(index as usize) {
        Some(r) => {
            msg!("round {}", r.name);
           r.set_tier_allocation(tier_allocations)?;
        }
        None => {
            return err!(IDOProgramErrors::InvalidInDex);
        }
    }

    Ok(())
}


pub fn modify_round(
    ctx: Context<AdminModifier>,
    index: i32,
    name: String,
    duration_seconds: u32,
    class: RoundClass,
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    ido_account.modify_round(
        &index,
        &name,
        &duration_seconds,
        &class,
    )?;

    Ok(())
}

pub fn modify_rounds(
    ctx: Context<AdminModifier>,
    name_list: Vec<String>,
    duration_list: Vec<u32>,
    class_list: Vec<RoundClass>
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;

    require!(name_list.len() > 0, IDOProgramErrors::InvalidRounds);
    require!(  name_list.len() == duration_list.len(), IDOProgramErrors::InvalidRounds);

    ido_account.modify_rounds(
        &name_list,
        &duration_list,
        &class_list
    )?;

    Ok(())
}