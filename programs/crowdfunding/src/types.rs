
use anchor_lang::prelude::*;

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializeIdoParam {
    pub raise_token: String,
    pub rate: u32,
    pub open_timestamp: i64,
    pub allocation_duration: u32,
    pub fcfs_duration: u32,
    pub cap: u64,
    pub release_token: String,
    pub ido_id: u64,
}

