
use std::ops::{Add, Sub};

use anchor_lang::prelude::*;

use crate::{IDOProgramErrors, ReleaseItem, RoundClass, RoundItem, TierItem};

#[account]
pub struct IdoAccount {
    pub _closed: bool, //1
    pub _release_token_decimals: u8, //1
    pub _raise_token_decimals: u8, //1
    pub bump: u8, //1
    pub _rate: u32, // 4 decimal 10000
    pub ido_id: u64, //8
    pub _open_timestamp: i64, //4
    pub _participated_count: u32, //4
    pub _participated: u64, //8
    pub _cap: u64, //8
    pub _release_token: Pubkey, //32
    pub _raise_token: Pubkey, //32
    pub authority: Pubkey, //32
    pub _tiers: Vec<TierItem>, //4 +(4+ 32 + 2) * 10 
    pub _rounds: Vec<RoundItem>, //4 + 126*3
    pub _releases: Vec<ReleaseItem>, //4 + 12 * 10
}

impl IdoAccount  {
    pub fn create_ido(
        &mut self,
        admin: &Pubkey,
        raise_token: &Pubkey,
        decimals: &u8,
        rate: &u32,
        open_timestamp: &i64,
        allocation_duration: &u32,
        fcfs_duration: &u32,
        cap: &u64,
        ido_id: &u64,
        bump: &u8,
    ) -> Result<()> {
        self._raise_token = *raise_token;
        self._raise_token_decimals = *decimals;
        self._rate = *rate;
        self._open_timestamp = *open_timestamp;
        self._cap = *cap;
        self._closed = false;
        self.authority = *admin;
        self.ido_id = *ido_id;
        self.bump = *bump;
        self._release_token = Pubkey::default();   
        self.init_tier()?;
        self.init_rounds(allocation_duration, fcfs_duration)?;
        Ok(())
    }

    pub fn init_tier(&mut self) -> Result<()> {
        self._tiers = vec![];
        self.add_tier(TierItem {
            name: String::from("Lottery Winners"),
            allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 100"),
            allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 200"),
            allocated_count: 0,
        });
        Ok(())
    }
    pub fn init_rounds(&mut self, allocation_duration: &u32, fcfs_duration: &u32) -> Result<()> {
        //check lai logic add round chỗ constructor của JD tier_allocations
        self._rounds = vec![];
        self.add_round(RoundItem {
            name: String::from("Allocation"),
            duration_seconds: *allocation_duration,
            class: RoundClass::Allocation,
            tier_allocations: vec![],
        });

        self.add_round(RoundItem {
            name: String::from("FCFS - Prepare"),
            duration_seconds: 900,
            class: RoundClass::FcfsPrepare,
            tier_allocations: vec![],

        });

        self.add_round(RoundItem {
            name: String::from("FCFS"),
            duration_seconds: *fcfs_duration,
            class: RoundClass::Fcfs,
            tier_allocations: vec![],
  
        });

        Ok(())
    }

    pub fn add_tier(&mut self, tier: TierItem) {
        self._tiers.push(tier);
    }

    pub fn add_round(&mut self, round: RoundItem) {
        self._rounds.push(round);
    }

    pub fn set_closed(&mut self, close: &bool) -> Result<()> {
        self._closed = *close;
        Ok(())
    }

    pub fn set_cap(&mut self, cap: &u64) -> Result<()> {
        self._cap = *cap;
        Ok(())
    }

    pub fn set_releases( &mut self, from_timestamps: &Vec<i64>, to_timestamps: &Vec<i64>, percents: &Vec<u16>,) -> Result<()> {
        self._releases = vec![];
        //get info Ido from account address
        for (i, from_timestamp) in from_timestamps.iter().enumerate() {
            self._releases.push(ReleaseItem {
                from_timestamp: *from_timestamp,
                to_timestamp: to_timestamps[i],
                percent: percents[i],
            });
        }
        Ok(())
    }

    pub fn set_release_token(
        &mut self,
        token: &Pubkey,
        token_decimals: &u8,
    ) -> Result<()> {
        self._release_token = *token;
        self._release_token_decimals = *token_decimals; //hardcode
        Ok(())
    }

    pub fn modify_round(
        &mut self,
        index: i32,
        name: String,
        duration_seconds: u32,
        class: RoundClass,
    ) -> Result<()> {
        match self._rounds.get_mut(index as usize) {
            Some(r) => {
                r.name = name;
                r.duration_seconds = duration_seconds;
                r.class = class;
            }
            None => {
                return err!(IDOProgramErrors::InvalidRounds);
            }
        }
        Ok(())
    }

    pub fn modify_rounds(
        &mut self,
        name_list: &Vec<String>,
        duration_list: &Vec<u32>,
        class_list: &Vec<RoundClass>,
    ) -> Result<()> {
        self._rounds = vec![];
        //push round into ido_account._rounds
        for (i, name) in name_list.iter().enumerate() {
            self.add_round(RoundItem {
                name: name.to_string(),
                duration_seconds: duration_list[i],
                class: class_list[i].clone(),
                tier_allocations: vec![],
            });
        }
        Ok(())
    }

   

    pub fn set_rate(&mut self, rate: &u32) -> Result<()> {
        self._rate = *rate;
        Ok(())
    }

    pub fn set_open_timestamp(&mut self, open_timestamps: &i64) -> Result<()> {
        self._open_timestamp = *open_timestamps;
        Ok(())
    }



    pub fn close_timestamp(&self) -> i64 {
        let mut ts = self._open_timestamp;
        let rounds = &self._rounds;
        for (_, round) in rounds.iter().enumerate() {
            ts = ts.add(round.duration_seconds as i64);
        }
        ts
    }

    pub fn fcfs_timestamp(&self) -> i64 {
        let mut ts = self._open_timestamp;
        let rounds = &self._rounds;
        for (_, round) in rounds.iter().enumerate() {
            match round.class {
                RoundClass::FcfsPrepare => {
                    return ts;
                }
                RoundClass::Fcfs => {
                    return ts;
                }
                _ => {
                    ts = ts.add(round.duration_seconds as i64);
                }
            }
        }
         ts
    }

    pub fn _is_close(&self) -> bool {
        let close_timestamp = self.close_timestamp();
     
        //get block time stamp
        let now_ts = Clock::get().unwrap().unix_timestamp ;
        //check close time  and pr
        if self._closed || now_ts >= close_timestamp || self._participated >= self._cap {
            return true;
        }
        false
    }
    pub fn bump(&self) -> u8 {
        self.bump
    }

    pub fn update_allocate_count(&mut self, index: &usize, remove: &bool) -> Result<()> {
        match self._tiers.get_mut(*index) {
            Some(tier) => {
                if !remove {
                    tier.allocated_count = tier.allocated_count.add(1);
                } else {
                    if tier.allocated_count > 0 {
                        tier.allocated_count = tier.allocated_count.sub(1);
                    }
                }
            }
            None => {
                return err!(IDOProgramErrors::InvalidRounds);
            }
        }
        Ok(())
    }

    
}