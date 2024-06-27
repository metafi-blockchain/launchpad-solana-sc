
use anchor_lang::prelude::*;

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum RoundClass {
    Allocation,
    FcfsPrepare,
    Fcfs,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RoundItem {
    pub duration_seconds: u32, //4
    pub name: String, //4 + 32
    pub class: RoundClass,  //1 + 1
    pub tier_allocations: Vec<u64>, //4 + 8*10 
}

impl RoundItem {
    pub fn get_tier_allocation(&self, index: u8) -> u64 {
        let tier_allocations = &self.tier_allocations;
        match tier_allocations.get(index as usize) {
            Some(&al) => {
                 al
            }
            None => {
                 0
            }
        }
    }
    pub fn set_tier_allocation(&mut self, tier_allocations: Vec<u64>)->Result<()> {
        self.tier_allocations = tier_allocations;
        Ok(())
    }
}