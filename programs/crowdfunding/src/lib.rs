

use anchor_lang::prelude::borsh::BorshDeserialize;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;


declare_id!("6KMVQWmTXpd36ryMi7i91yeLsgM6S4BiaTX3UczEkvqq");

#[program]
pub mod crowdfunding {
    use std::str::FromStr;
    use spl_token::instruction::transfer; 

    use super::*;

    pub fn initialize(
        ctx: Context<CreateIdoAccount>,
        raise_token: String,
        rate: u16,
        open_timestamp: u32,
        allocation_duration: u32,
        fcfs_duration: u32,
        cap: u64,
        release_token: String,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;

        ido_account._raise_token = Pubkey::from_str(&raise_token).unwrap();
        ido_account._rate = rate;
        ido_account._open_timestamp = open_timestamp;

        ido_account._cap = cap;
        ido_account._closed = false;
        ido_account._owner = *ctx.accounts.user.key;

        ido_account._release_token = Pubkey::from_str(&release_token).unwrap();

        //add tier
        ido_account._tiers.push(TierItem {
            name: String:: from("Lottery Winners"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name: String:: from("Top 100"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name:String:: from( "Top 200"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name: String:: from("Top 300"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name: String:: from("Top 400"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name: String:: from("Top 500"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name: String:: from("Top 600"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name: String:: from("Top 700"),
            allocated: vec![],
            // allocated_count: 0,
        });
        
        ido_account._tiers.push(TierItem {
            name: String:: from("Top 800"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name:String:: from( "Top 900"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account._tiers.push(TierItem {
            name:String:: from( "Top 1000"),
            allocated: vec![],
            // allocated_count: 0,
        });

        //check lai logic add round chỗ constructor của JD tier_allocations
        //add rounds
        ido_account._rounds.push(RoundItem {
            name: String:: from("Allocation"),
            duration_seconds: allocation_duration,
            class: RoundClass::Allocation,
            tier_allocations: vec![],
            participated: vec![],
        });

        ido_account._rounds.push(RoundItem {
            name: String:: from("FCFS - Prepare"),
            duration_seconds: 900,
            class: RoundClass::FcfsPrepare,
            tier_allocations: vec![],
            participated: vec![],
        });

        ido_account._rounds.push(RoundItem {
            name: String:: from("FCFS"),
            duration_seconds: fcfs_duration,
            class: RoundClass::Fcfs,
            tier_allocations: vec![],
            participated: vec![],
        });
        msg!("Create account success!");
        Ok(())
    }

    //todo: check lại logic phan tiers khai bao
    pub fn modify_rounds(ctx: Context<Modifier>, name_list : Vec<String>, duration_list: Vec<u32>, class_list: Vec<RoundClass>)-> ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        //check name_list
        if name_list.is_empty() {
            return Err(ProgramError::InvalidArgument);
        }
        //check size
        if name_list.len() != duration_list.len() || name_list.len() != class_list.len() {
            return Err(ProgramError::InvalidArgument);
        }
        //delete round
        ido_account._rounds = vec![];

        //push round into ido_account._rounds
        for (i, name) in name_list.iter().enumerate() {
            ido_account._rounds.push(RoundItem {
                name: name.to_string(),
                duration_seconds: duration_list[i],
                class: class_list[i].clone(),
                tier_allocations: vec![],
                participated: vec![],
            });
        }
        Ok(())
    }


    pub fn modify_round(ctx: Context<Modifier>, index: u16, name: String, duration_seconds: u32, class: RoundClass)-> ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;

        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        // check round index
        if ido_account._rounds.len() < index as usize {
            return Err(ProgramError::InvalidArgument);
        }
        
        let r = &mut ido_account._rounds[index as usize];
        r.name = name;
        r.duration_seconds = duration_seconds;
        r.class = class;
        Ok(())
    }

    pub fn modify_round_allocations(ctx: Context<ModifyTier>, index: u32, tier_allocations: Vec<u64>)-> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        // check round index
        if ido_account._rounds.len() < index as usize {
            return Err(ProgramError::InvalidArgument);
        }
        //get info Ido from account address
        let r = &mut ido_account._rounds[index as usize];
        r.tier_allocations = tier_allocations;


        Ok(())
    }

    pub fn modify_tier(ctx: Context<Modifier>, index: u16, name: String) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if ido_account._tiers.len() < index as usize {
            return Err(ProgramError::InvalidArgument);
        }
        //get info Ido from account address
        let tier = &mut ido_account._tiers[index as usize];

        tier.name = name;
        Ok(())
    }

    pub fn modify_tiers(ctx: Context<Modifier>, name_list : Vec<String> ) -> ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;

        //check name_list
        if name_list.is_empty() {
            return Err(ProgramError::InvalidArgument);
        }
        //check owner 
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        //delete tier 
        ido_account._tiers = vec![];

        //push tier into ido_account._tiers
        for (_, name) in name_list.iter().enumerate() {
            ido_account._tiers.push(TierItem {
                name: name.to_string(),
                allocated: vec![],
                // allocated_count: 0,
            });
        }   
        
        Ok(())
    }

    /**
     * them hoac remove address vao allocation cua tier
     */
    pub fn modify_tier_allocated(
        ctx: Context<Modifier>,
        index: u16,
        addresses: Vec<String>,
        remove: bool,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if ido_account._tiers.len() < index as usize {
            return Err(ProgramError::InvalidArgument);
        }

        let tier = &mut ido_account._tiers[index as usize];
        

        for (_, address) in addresses.iter().enumerate() {
            let address = Pubkey::from_str(address).unwrap();

            let al = {
                AllocateTier {
                    address,
                    allocated: !remove,
                }
            };
            tier.add_allocated(al);

            // let mut check_exits = false;
            // let mut index = 0;
            
            
            // for (i, item) in tier.allocated.iter().enumerate() {
            //     if item.address == address {
            //         check_exits = true;
            //         index = i;
            //         break;
            //     }
            // }

            // if check_exits {
            //     tier.allocated[index].allocated = !remove;
                
            // } else {
            //     tier.allocated.push(AllocateTier {
            //         address,
            //         allocated: !remove,
            //     });
            // }

            // if !remove {
            //     tier.allocated_count += 1;
            // } else {
            //     if tier.allocated_count > 0 {
            //         tier.allocated_count -= 1;
            //     }
            // }
        }

        Ok(())
    }

    pub fn setup_release_token(
        ctx: Context<SetupReleaseToken>,
        token: String,
        pair: String,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        //get info Ido from account address
        ido_account._release_token = Pubkey::from_str(&token).unwrap();
        ido_account._release_token_pair = Pubkey::from_str(&pair).unwrap();
        Ok(())
    }

    pub fn participate(ctx: Context<Participate>, amount: u64) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        let system_program = &mut ctx.accounts.system_program;

        
        //transfer token
        let _ix = transfer(
            &system_program.key(),
            &user.key(),
            &ido_account._release_token,
            &user.key,
            &[],
            amount,
        )?;
        Ok(())
    
    }

    pub fn setup_releases(ctx: Context<SetupReleases>, from_timestamps: Vec<u32> , to_timestamps: Vec<u32>,  percents : Vec<u16>) ->ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
       
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }

        //check size
        if from_timestamps.len() != to_timestamps.len() || from_timestamps.len() != percents.len() {
            return Err(ProgramError::InvalidArgument);
        }

        ido_account._releases = vec![];
        //get info Ido from account address
        for (i, from_timestamp) in from_timestamps.iter().enumerate() {
        
            ido_account._releases.push(ReleaseItem{
                from_timestamp: *from_timestamp,
                to_timestamp: to_timestamps[i],
                percent: percents[i],
                claimed: vec![]
            });
        }
        Ok(())
    }

    pub fn set_closed(ctx: Context<Modifier>, close: bool) ->ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        ido_account._closed = close;
        Ok(())
    }

    pub fn set_cap(ctx: Context<Modifier>, cap: u64) ->ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        ido_account._cap = cap;
        Ok(())
    }

    //doing check lai ham nay la with draw balance cua SC ve vi ca nhan
    pub fn transfer_native_token(ctx: Context<TransferNativeToken>, amount: u64, to: Pubkey )->ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        // let system_program = &mut ctx.accounts.system_program;
        //check owner
        if ido_account._owner != *user.key {
            return Err(ProgramError::InvalidAccountOwner);
        }
        //transfer token
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &user.key(),
            &to,
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.ido_info.to_account_info()
            ]
        )?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateIdoAccount<'info> {
    #[account(init, payer = user, space = 10000)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct IdoAccountInfo {
    pub _raise_token: Pubkey,
    pub _rate: u16,
    pub _open_timestamp: u32,
    pub _cap: u64,
    pub _participated: u64,
    pub _participated_count: u32,
    pub _closed: bool,
    pub _owner: Pubkey,

    pub _release_token: Pubkey,
    pub _release_token_pair: Pubkey,

    pub _tiers: Vec<TierItem>,
    pub _rounds: Vec<RoundItem>,
    pub _releases: Vec<ReleaseItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum RoundClass {
    Allocation ,
    FcfsPrepare,
    Fcfs,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RoundItem {
    pub name: String,
    pub duration_seconds: u32,
    pub class: RoundClass,
    pub tier_allocations: Vec<u64>,
    pub participated: Vec<Participated>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Participated {
    pub address: Pubkey,
    pub amount: u64,
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ReleaseItem {
    from_timestamp:  u32,
    to_timestamp: u32,
    percent: u16,
    claimed: Vec<ClaimedAmount>
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ClaimedAmount{
    pub address: Pubkey,
    pub amount: u64, 
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TierItem {
    pub name: String,
    pub allocated: Vec<AllocateTier>,
    // pub allocated_count: u64,
}
impl  TierItem {
     pub fn add_allocated(&mut self, al: AllocateTier){

        let  allocated = self.allocated.clone();
        //check al in allocated
        let mut check_exits = false;
        let mut index: usize = 0;
        for (i, item) in allocated.iter().enumerate() {
            if item.address == al.address {
                check_exits = true;
                index = i;
                break;
            }
        }
        //if exits update 
        if check_exits {
            self.allocated[index].allocated = al.allocated;
        } else {
            self.allocated.push(al);
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AllocateTier {
    pub address: Pubkey,
    pub allocated: bool,
}

#[derive(Accounts)]
pub struct SetupReleaseToken<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyTier<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct  Participate<'info>{
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct  SetupReleases<'info>{
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct  Modifier<'info>{
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct  TransferNativeToken<'info>{
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}


// #[derive(Accounts)]
// pub struct ModifyTierAllocated<'info>{
//     #[account(mut)]
//     pub ido_info: Account<'info, IdoAccountInfo>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>
// }
