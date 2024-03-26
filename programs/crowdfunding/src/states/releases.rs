

use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ReleaseItem {
    pub percent: u16,
    pub from_timestamp: u32,
    pub to_timestamp: u32,
 
}