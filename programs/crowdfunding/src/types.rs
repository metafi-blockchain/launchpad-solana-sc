
use anchor_lang::prelude::*;

use crate::RoundClass;

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializeIdoParam {
    pub ido_id: u64,
    pub raise_token: Pubkey,
    pub rate: u32,
    pub open_timestamp: i64,
    pub allocation_duration: u32,
    pub fcfs_duration: u32,
    pub cap: u64,

}
#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SetupUserTierAllocationParam {
    pub tier: u8,
    pub address: Pubkey,
    pub remove: bool
}
#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]

pub struct ModifyRoundsParam{
    pub name_list: Vec<String>,
    pub duration_list: Vec<u32>,
    pub class_list: Vec<RoundClass>
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]

pub struct ModifyRoundParam{
    pub round_index: i32,
    pub name: String,
    pub duration_seconds: u32,
    pub class: RoundClass,
}
#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ModifyRoundAllocation{
    pub round_index: u8,
    pub tier_allocations: Vec<u64>,
}