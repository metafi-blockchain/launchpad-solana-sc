use std::ops::Sub;

use anchor_lang::prelude::*;
use solana_safe_math::SafeMath;

use crate::IDOProgramErrors;

#[account]
pub struct PdaUserStats {
    pub allocated: bool,         //1
    pub bump: u8,                //1
    pub tier_index: u8,          //1
    pub participate: Vec<ParticipateItem>, //4 + 9
    pub claims: Vec<ClaimItem>,  //4 + 9
    pub address: Pubkey,         //32
}

impl PdaUserStats {
    pub fn init_user_pda(
        &mut self,
        tier_index: &u8,
        address: &Pubkey,
        // owner: &Pubkey,
        allocated: &bool,
        bump: &u8,
    ) -> Result<()> {
        self.tier_index = *tier_index;
        self.address = *address;
        // self.owner = *owner;
        self.tier_index = *tier_index;
        self.allocated = *allocated;
        self.bump = *bump;
        Ok(())
    }
    pub fn update_allocate(&mut self, tier_index: &u8, allocated: &bool) {
        self.tier_index = *tier_index;
        self.allocated = *allocated;
    }


    pub fn user_participate(&mut self, round: u8, participate_amount: u64) -> Result<()> {
        let round_index  =  round.sub(1);
        match self.participate.get_mut(round_index as usize) {
            Some(p) => {
                self.participate[round_index as usize].amount = p.amount.safe_add(participate_amount).unwrap();
            }
            None => {
                self.participate.push(ParticipateItem {
                    round: round_index,
                    amount: participate_amount,
                });
            }
            
        }
        Ok(())
    }
    pub fn get_amount_participate_round(&self, index: u8) -> Result<u64> {
        for p in self.participate.iter() {
            if p.round == index {
                return Ok(p.amount);
            }
        }
        Ok(0)
    }


    pub fn get_total_participate(&self)-> Result<u64>{
        let mut total: u64 = 0;
        for p in self.participate.iter() {
            total = total.safe_add(p.amount).unwrap();
        }
        Ok(total)
    }

    pub fn user_claim(&mut self, index: u8, claim_amount: u64) -> Result<()> {
        match self.claims.get_mut(index as usize) {
            Some(c) => {
                c.amount = c.amount.safe_add(claim_amount).unwrap();
            }
            None => {
                self.claims.push(ClaimItem {
                    release: index,
                    amount: claim_amount,
                });
            }
        }
        Ok(())
    }
    pub fn get_total_claim(self)-> Result<u64>{
        let mut total: u64 = 0;
        for c in self.claims.iter() {
            total = total.safe_add(c.amount).unwrap();
        }
        Ok(total)
    }

    pub fn get_amount_claim_release_round(&self, index: u8) -> Result<u64> {

        for c in self.claims.iter() {
            if c.release == index {
                return Ok(c.amount);
            }
        }
        Ok(0) 
    }


    pub fn get_size(&self)-> usize{
        let size = 8 + 75  + self.claims.len()  * 9;
        size
    }
   

    pub fn safe_deserialize(mut data: &[u8]) -> Result<Self> {
        let result = Self::try_deserialize(&mut data)?;

        Ok(result)
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<Self> where {
        let data = &a.data.borrow_mut();
        let ua = Self::safe_deserialize(data).map_err(|_| IDOProgramErrors::CannotParseData)?;
        Ok(ua)
    }

}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ClaimItem {
    release: u8, //1
    amount: u64, //8
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ParticipateItem {
    round: u8, //
    amount: u64, //8
}