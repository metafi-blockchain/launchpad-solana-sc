
use anchor_lang::prelude::*;

use crate::{ AuthRole, AuthorityRole, IDOProgramErrors, IdoAccount, ModifyRoundAllocationParam, ModifyRoundParam, ModifyRoundsParam, ModifyTierName, SetupReleaseTokenParam, TierItem};

use crate::{ AUTHORITY_IDO, OPERATOR_ROLE};

#[derive(Accounts)]
pub struct AdminModifier<'info> {
    #[account(
        mut,
        constraint = ido_account.authority == operator_pda.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump = ido_account.bump)]
    pub ido_account:Box<Account<'info, IdoAccount>>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_pda.bump,
        constraint = operator_pda.has_authority(authority.key(), AuthRole::Operator ) == true @ IDOProgramErrors::OnlyOperatorAllowed,
    )]
    pub operator_pda: Account<'info, AuthorityRole>,
    #[account(signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_setup_releases(
    ctx: Context<AdminModifier>,
    param: SetupReleaseTokenParam,
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
   
    let SetupReleaseTokenParam {
        from_timestamps,
        to_timestamps,
        percents,
    } = param;
    
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

pub fn handle_set_closed(ctx: Context<AdminModifier>, close: bool) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    ido_account.set_closed( &close)?;
    Ok(())
}

pub fn handle_set_cap(ctx: Context<AdminModifier>, cap: u64) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    ido_account.set_cap(&cap)?;
    Ok(())
}

pub fn handle_set_rate(ctx: Context<AdminModifier>, rate: u32) -> Result<()> {
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

pub fn handle_modify_tier(ctx: Context<AdminModifier>, param: ModifyTierName) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;


    let ModifyTierName {
        tier_index,
        name,
    } = param;
    match ido_account._tiers.get_mut(tier_index as usize) {
        Some(tier) => {
            tier.name = name;
        }
        None => {
            return err!(IDOProgramErrors::InValidTier);
        }
    }
    Ok(())
}

pub fn handle_modify_round_allocations(
    ctx: Context<AdminModifier>,
    param: ModifyRoundAllocationParam
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;

    let ModifyRoundAllocationParam {
        round_index,
        tier_allocations,
    } = param;
    
    match ido_account._rounds.get_mut(round_index as usize) {
        Some(r) => {
           r.set_tier_allocation(tier_allocations)?;
        }
        None => {
            return err!(IDOProgramErrors::InvalidRounds);
        }
    }

    Ok(())
}


pub fn handle_modify_round(
    ctx: Context<AdminModifier>,
    param: ModifyRoundParam,
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
   
    let ModifyRoundParam {
        round_index,
        name,
        duration_seconds,
        class
    } = param;

    ido_account.modify_round( round_index,
        name,
        duration_seconds,
        class,
    )?;

    Ok(())
}

pub fn handle_modify_rounds(
    ctx: Context<AdminModifier>,
    param: ModifyRoundsParam
) -> Result<()> {


    let ido_account = &mut ctx.accounts.ido_account;

    let ModifyRoundsParam {
        name_list,
        duration_list,
        class_list
    } = param;
    require!(name_list.len() > 0, IDOProgramErrors::InvalidRounds);
    require!(  name_list.len() == duration_list.len(), IDOProgramErrors::InvalidRounds);

    ido_account.modify_rounds(
        &name_list,
        &duration_list,
        &class_list
    )?;

    Ok(())
}