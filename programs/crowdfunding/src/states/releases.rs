

use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ReleaseItem {
    pub percent: u16,  //4 decimals
    pub from_timestamp: i64,
    pub to_timestamp: i64,
 
}