use std::ops::{Add, Sub};

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_safe_math::SafeMath;
use crate::{IdoAccount, PdaUserStats, RoundClass};


const PERCENT_SCALED_DECIMALS: u64 = 10000;
const RATE_DECIMALS : u64 = 1000000;

pub fn _get_allocation(
    ido_account: &IdoAccount,
    user_pda: &PdaUserStats,
    release_token_account: &TokenAccount, 
    index: usize,
) -> (i64, i64, u16, u64, u64, u64, u64, u8) {


    match ido_account._releases.get(index) {
        Some(r) => {
            let _rate: u32 = ido_account._rate;
            let mut status: u8 = 0;
            let mut remaining: u64 = 0;
            let percent: u16 = r.percent;
            let from_timestamp: i64 = r.from_timestamp;
            let to_timestamp: i64 = r.to_timestamp;
            let participated: u64 = user_pda.participate_amount;
            let raise_decimals: u8 = ido_account._raise_token_decimals;
            let release_decimals: u8 = ido_account._release_token_decimals;
            msg!("participated: {}",participated);
            let mut total: u64 = participated
                .safe_mul(_rate as u64)
                .unwrap()
                .safe_div(RATE_DECIMALS)
                .unwrap()
                .safe_mul(percent as u64)
                .unwrap()
                .safe_div(PERCENT_SCALED_DECIMALS)
                .unwrap();
            msg!("total: {}",total);
            if raise_decimals > release_decimals {
                let base: u32 = 10;
                total = total.safe_div(base.safe_pow(raise_decimals.sub(release_decimals)as u32).unwrap() as u64).unwrap();
            }

            if release_decimals > raise_decimals {  
                let base: u32 = 10;
                total = total.safe_mul(base.safe_pow(release_decimals.sub(raise_decimals) as u32).unwrap() as u64).unwrap();
            }

            let mut claimable = total;
            msg!("claimable: {}",claimable);
            let now_ts = Clock::get().unwrap().unix_timestamp ;

            match (to_timestamp > from_timestamp) && (now_ts < to_timestamp)  {
                true => {
                    let mut elapsed = 0;
                    if now_ts > from_timestamp {
                        elapsed = now_ts.sub(from_timestamp);
                    }
                    let duration = to_timestamp.sub(from_timestamp);
                    claimable = total
                        .safe_mul(elapsed as u64)
                        .unwrap()
                        .safe_div(duration as u64)
                        .unwrap();
                }
                false => (),
            }
          
            let claimed = user_pda.get_amount_claim_release_round(index as u16).unwrap(/*None*/);
            msg!("claimed: {}",claimed);
            if claimed < claimable {
                remaining = claimable.safe_sub(claimed).unwrap();
            }   
            msg!("remaining: {}",remaining);

            let native_token_pub = Pubkey::default();
            // //check _release_token is equal publich key 1nc1nerator11111111111111111111111111111111
            if ido_account._release_token != native_token_pub {
                if from_timestamp == 0 || now_ts > from_timestamp {
                    status = 1;

                    //check balance release token account > 0
                    if release_token_account.amount == 0 {
                        status = 2;
                    }
                    //check balance release pair token account > 0  //doing
                    if remaining == 0 {
                        status = 2;
                    }  
                }
            }
             (
                from_timestamp,
                to_timestamp,
                percent,
                claimable,
                total,
                claimed,
                remaining,
                status,
            )
        }
        None => {
            msg!("Invalid release index");
             (0, 0, 0, 0, 0, 0, 0, 0)
        }
    }
}

pub fn _info_wallet( ido_account:&mut IdoAccount,  user_pda: &mut PdaUserStats) -> (u8, u8, u8, String, i64) {
    
    let mut round = 0;
    let mut round_state = 4;
    let mut round_state_text = String::from("");
    let mut round_timestamp = 0;
    let is_close =  ido_account._is_close();
    let tier: u8 = if user_pda.allocated  { user_pda.clone().tier_index } else { 0 };

    if !is_close {
        let mut ts = ido_account._open_timestamp;
        let now_ts = Clock::get().unwrap().unix_timestamp;
        if now_ts < ts {
            round_state = 0;
            round_state_text = String::from("Allocation Round <u>opens</u> in:");
            round_timestamp = ts;
        } else {
            let rounds = ido_account._rounds.clone();

            for (i, _round) in rounds.iter().enumerate() {
                round = i.add(1);
                ts = ts.add(_round.duration_seconds as i64);
                if now_ts < ts {
                    match _round.class {
                        RoundClass::Allocation => {
                            round_state = 1;
                            round_state_text =
                                String::from("Allocation Round <u>closes</u> in:");
                            round_timestamp = ts;
                        }
                        RoundClass::FcfsPrepare => {
                            round_state = 2;
                            round_state_text = String::from("FCFS Round <u>opens</u> in:");
                            round_timestamp = ts;
                        }
                        RoundClass::Fcfs => {
                            round_state = 3;
                            round_state_text = String::from("FCFS Round <u>closes</u> in:");
                            round_timestamp = ts;
                        }
                    }
                    break;
                }
            }
        }
    }

     (
        tier,
        round.try_into().unwrap() ,
        round_state,
        round_state_text,
        round_timestamp,
    )
}

pub fn get_allocation_remaining(ido_account:&mut IdoAccount, user_pda: &PdaUserStats ,round: &u8 ) -> u64 {

    let tier =  user_pda.tier_index;
    msg!("tier user {} ",tier );
    if *round == 0 || tier == 0 {
        return 0;
    }
   

    let round_index = round.sub(1) as usize;
    let _tier_index = tier;
    let rounds = ido_account._rounds.clone();
    

    if user_pda.allocated {
        match rounds.get(round_index) {
            Some(round) => {
                let participated = user_pda.participate_amount;
                let allocated = round.get_tier_allocation(_tier_index);
                msg!("allocated: {}",allocated);
                if participated < allocated {
                    return allocated.safe_sub(participated).unwrap();
                }
            }
            None => {
                return 0;
            }
        }  
    }
     0
}



pub fn _transfer_token_from_ido<'a>(data: &'a TokenTransferParams) -> Result<()> {
    let transfer_instruction = anchor_spl::token::Transfer {
        from: data.source.to_account_info(),
        to: data.destination.to_account_info(),
        authority: data.authority.to_account_info(),
    };
    let cpi_program = data.token_program.to_account_info();
    let signer = &[data.authority_signer_seeds];
    let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction).with_signer(signer);
    anchor_spl::token::transfer(cpi_ctx, data.amount)?;
    Ok(())
}

pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    /// CHECK: account checked in CPI
    pub source: AccountInfo<'a>,
    /// destination
    /// CHECK: account checked in CPI
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    /// CHECK: account checked in CPI
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    /// CHECK: account checked in CPI
    pub token_program: AccountInfo<'a>,
}

