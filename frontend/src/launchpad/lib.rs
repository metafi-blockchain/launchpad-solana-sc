use anchor_lang::prelude::*;
use solana_safe_math::{SafeMath};
use anchor_lang::solana_program::entrypoint::ProgramResult;


declare_id!("DCe5ipqT9R1aBQZzoqWeqwCN3ewCf4YhmoKcrneStS42");

#[program]
pub mod ido {
    use super::*;

    pub fn create_ido_address(ctx:  Context<CreateIdoAccount>, raiseToken: String,rate: String, startTime: u64, endTime: u64,  cap: u64) -> ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;

        ido_account.owner = *ctx.accounts.user.key;
        ido_account._raiseToken = raiseToken;
        ido_account._rate = rate;
        ido_account._startTime = startTime;
        ido_account._endTime = endTime;
        ido_account._cap = cap;
        ido_account._closed = false;
        Ok(())
    }

}


#[derive(Accounts)]
pub struct CreateIdoAccount<'info> {
    #[account(init, payer = user, space = 10000), bump]
    pub account: Account<'info, IdoInfo>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct IdoInfo{
    pub _raiseToken : String,
    pub _rate : u64,
    pub _openTimestamp: u64,
    pub _cap: u64,
    pub _participated: u64,
    pub _participatedCount: u64,
    pub _closed: bool
}
enum RoundClass { ALLOCATION, FcfsPrepare, FCFS }

pub struct Round {
    pub name: String,
    pub durationSeconds: u64,
    pub user_address: Pubkey
}

pub struct ItemStruct {
    pub amount: u64,
    pub user_address: Pubkey
}



#[derive(Accounts)]
struct Release {
    pub fromTimestamp u64,
    pub toTimestamp u64,
    pub percent u8,   
    pub claimed Vec<ItemStruct>                   //2 DECIMALS, eg. 33.33% = 3333
}
