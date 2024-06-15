use anchor_lang::prelude::*;
use solana_safe_math::SafeMath;

use crate::IDOProgramErrors;

#[account]
pub struct PdaUserStats {
    pub allocated: bool,         //1
    pub bump: u8,                //1
    pub tier_index: u8,          //1
    pub participate_amount: u64, //16
    pub claims: Vec<ClaimItem>,  //4 + (12*16)
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
    pub fn user_update_participate(&mut self, participate_amount: u64) -> Result<()> {
        self.participate_amount = self
            .participate_amount
            .safe_add(participate_amount)
            .unwrap();
        Ok(())
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
   

    pub fn safe_deserialize(mut data: &[u8]) -> Result<Self> {
        let result = Self::try_deserialize(&mut data)?;

        Ok(result)
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<Self> where {
        let data = &a.data.borrow_mut();
        let ua = Self::safe_deserialize(data).map_err(|_| IDOProgramErrors::CannotParseData)?;
        Ok(ua)
    }
    //try serialize data to array
    // fn try_to_vec(&self) -> Result<Vec<u8>> {
    //     let mut data = vec![];
    //     self.try_serialize(&mut data)?;
    //     Ok(data)
    // }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ClaimItem {
    release: u8,
    amount: u64,
}
