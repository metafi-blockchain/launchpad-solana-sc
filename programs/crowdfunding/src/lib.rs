
mod instructions;
mod states;
mod utils;
mod errors;
mod events;

use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;


declare_id!("A7HQd8NLQAj5DRxZUXS5vNkpUfDhnDRkHS8KhrP8eP1t");


#[program]
pub mod crowdfunding {

    use super::*;
    pub use instructions::*;
    pub use states::*;
    pub use utils::*;
    pub use errors::*;
    pub use events::*;

    /// Seed for tran authority seed
    pub const AUTHORITY_IDO: &[u8] = b"ido_pad";
    pub const AUTHORITY_ADMIN: &[u8] = b"admin_ido";
    pub const AUTHORITY_USER: &[u8] = b"wl_ido_pad";


    pub fn initialize(
        ctx: Context<InitializeIdoAccount>,
        raise_token: String,
        rate: u32,
        open_timestamp: i64,
        allocation_duration: u32,
        fcfs_duration: u32,
        cap: u64,
        release_token: String,
        ido_id: u64,
    ) -> Result<()> {

        instructions::initialize(ctx, raise_token, rate, open_timestamp, allocation_duration, fcfs_duration, cap, release_token, ido_id)
    }

    pub fn update_admin_ido( ctx: Context<UpdateAdminIdo>, admin_address : Pubkey)->Result<()>{
        instructions::update_admin_ido(ctx, admin_address)
    }

    pub fn modify_rounds(
        ctx: Context<AdminModifier>,
        name_list: Vec<String>,
        duration_list: Vec<u32>,
        class_list: Vec<RoundClass>
    ) -> Result<()> {
        instructions::modify_rounds(ctx, name_list, duration_list, class_list)
    }

    pub fn modify_round(
        ctx: Context<AdminModifier>,
        index: i32,
        name: String,
        duration_seconds: u32,
        class: RoundClass,
    ) -> Result<()> {
        instructions::modify_round(ctx, index, name, duration_seconds, class)
    }

    pub fn modify_round_allocations(
        ctx: Context<AdminModifier>,
        index: u8,
        tier_allocations: Vec<u64>,
    ) -> Result<()> {
        instructions::modify_round_allocations(ctx, index, tier_allocations)
    }

    pub fn modify_tier(ctx: Context<AdminModifier>, index: u32, name: String) -> Result<()> {
        instructions::modify_tier(ctx, index, name)
    }

    pub fn modify_tiers(ctx: Context<AdminModifier>, name_list: Vec<String>) -> Result<()> {
        instructions::modify_tiers(ctx, name_list)
    }

    pub fn modify_tier_allocated_one(
        ctx: Context<ModifyTierAllocatedOne>,
        index: u8,
        address: Pubkey,
        remove: bool,
    ) -> Result<()> {
        instructions::modify_tier_allocated_one(ctx, index, address, remove)
    }

    pub fn setup_release_token( ctx: Context<SetupReleaseToken>,release_token: Pubkey) -> Result<()> {
        instructions::setup_release_token(ctx, release_token)
    }

    pub fn setup_releases(
        ctx: Context<AdminModifier>,
        from_timestamps: Vec<i64>,
        to_timestamps: Vec<i64>,
        percents: Vec<u16>,
    ) -> Result<()> {
        instructions::setup_releases(ctx, from_timestamps, to_timestamps, percents)
    }

    pub fn set_closed(ctx: Context<AdminModifier>, close: bool) -> Result<()> {
        instructions::set_closed(ctx, close)
    }

    pub fn set_cap(ctx: Context<AdminModifier>, cap: u64) -> Result<()> {
        instructions::set_cap(ctx, cap)
    }

    pub fn set_rate(ctx: Context<AdminModifier>, rate: u32) -> Result<()> {
        instructions::set_rate(ctx, rate)
    }
    pub fn set_open_timestamp(ctx: Context<AdminModifier>, open_timestamp: i64) -> Result<()> {
        instructions::set_open_timestamp(ctx, open_timestamp)
    }

    // transferNativeToken
    // with draw token from pda of admin
    pub fn withdraw_native_token( ctx: Context<TransferNativeToken>, amount: u64, _to: Pubkey,
    ) -> Result<()> {
        instructions::withdraw_native_token(ctx, amount, _to)
    }

    //transferToken
    //with draw token  only admin who create pda withdraw token
    pub fn withdraw_token_from_pda(ctx: Context<WithdrawTokenFromPda>, amount: u64) -> Result<()> {
        instructions::withdraw_token_from_pda(ctx, amount)
    }

    //user join IDO
    pub fn participate(ctx: Context<Participate>, amount: u64) -> Result<()> {
        instructions::participate(ctx, amount)
    }

    pub fn claim(ctx: Context<ClaimToken>, index: u16) -> Result<()> {
        instructions::claim(ctx, index)
    }
  

}















