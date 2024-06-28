use anchor_lang::prelude::*;
/**
 *  event structure
 */

#[event]
pub struct ParticipateEvent {
    pub amount: u64,
    pub address: Pubkey,
}
#[event]
pub struct ClaimEvent {
    pub index: u8,
    pub address: Pubkey,
    pub claim: u64,
    pub timestamp: i64,
}
#[event]

pub struct WithdrawTokenEvent {
    pub amount: u64,
    pub address: Pubkey,
    pub timestamp: i64,
}


#[event]
pub struct ChangeOperatorWalletEvent {
    pub admin: Pubkey,
    pub operator_wallet: Pubkey,
    pub time: i64,
}