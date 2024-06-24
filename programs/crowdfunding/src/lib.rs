
mod instructions;
mod states;
mod utils;
mod errors;
mod events;
mod types;

use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;


declare_id!("A7HQd8NLQAj5DRxZUXS5vNkpUfDhnDRkHS8KhrP8eP1t");


#[program]
pub mod crowdfunding {

    use super::*;
    pub use instructions::*;
    pub use states::*;
    pub use types::*;
    pub use utils::*;
    pub use errors::*;
    pub use events::*;

    /// Seed for tran authority seed
    pub const AUTHORITY_IDO: &[u8] = b"ido_pad";
    pub const AUTHORITY_ADMIN: &[u8] = b"admin_ido";
    pub const AUTHORITY_USER: &[u8] = b"wl_ido_pad";


    pub fn initialize(
        ctx: Context<InitializeIdoAccount>,
        params: InitializeIdoParam
    ) -> Result<()> {
        instructions::handle_initialize(ctx, params)
    }

    pub fn update_admin_ido( ctx: Context<UpdateAdminIdo>, admin_address : Pubkey)->Result<()>{
        instructions::handle_update_admin_ido(ctx, admin_address)
    }

    pub fn modify_rounds(
        ctx: Context<AdminModifier>,
        param: ModifyRoundsParam
    ) -> Result<()> {
        instructions::handle_modify_rounds(ctx, param)
    }

    pub fn modify_round(
        ctx: Context<AdminModifier>,
        param: ModifyRoundParam,
    ) -> Result<()> {
        instructions::handle_modify_round(ctx, param)
    }

    pub fn modify_round_allocations(
        ctx: Context<AdminModifier>,
        param: ModifyRoundAllocationParam
    ) -> Result<()> {
        instructions::handle_modify_round_allocations(ctx, param)
    }

    pub fn modify_tier(ctx: Context<AdminModifier>, param: ModifyTierName) -> Result<()> {
        instructions::handle_modify_tier(ctx, param)
    }

    pub fn modify_tiers(ctx: Context<AdminModifier>, name_list: Vec<String>) -> Result<()> {
        instructions::modify_tiers(ctx, name_list)
    }

    pub fn modify_tier_allocated(
        ctx: Context<ModifyTierAllocatedOne>, param: SetupUserTierAllocationParam
    ) -> Result<()> {
        instructions::handle_modify_tier_allocated(ctx, param)
    }

    pub fn setup_release_token( ctx: Context<SetupReleaseToken>,release_token: Pubkey) -> Result<()> {
        instructions::setup_release_token(ctx, release_token)
    }

    pub fn setup_releases(
        ctx: Context<AdminModifier>,
        param: SetupReleaseTokenParam
    ) -> Result<()> {
        instructions::handle_setup_releases(ctx,param)
    }

    pub fn set_closed(ctx: Context<AdminModifier>, close: bool) -> Result<()> {
        instructions::handle_set_closed(ctx, close)
    }

    pub fn set_cap(ctx: Context<AdminModifier>, cap: u64) -> Result<()> {
        instructions::handle_set_cap(ctx, cap)
    }

    pub fn set_rate(ctx: Context<AdminModifier>, rate: u32) -> Result<()> {
        instructions::handle_set_rate(ctx, rate)
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
    pub fn participate_sol(ctx: Context<ParticipateSol>, amount: u64) -> Result<()> {
        instructions::participate_sol(ctx, amount)
    }

    pub fn claim(ctx: Context<ClaimToken>, index: u8) -> Result<()> {
        instructions::claim(ctx, index)
    }
  

}
