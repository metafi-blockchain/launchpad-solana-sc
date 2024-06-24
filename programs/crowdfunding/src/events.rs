use anchor_lang::prelude::*;
/**
 *  event structure
 */

#[event]
pub struct ParticipateEvent {
    pub amount: u64,
    pub address: String,
}
#[event]
pub struct ClaimEvent {
    pub index: u8,
    pub address: String,
    pub claim: u64,
    pub timestamp: i64,
}
#[event]

pub struct WithdrawTokenEvent {
    pub amount: u64,
    pub address: String,
    pub timestamp: i64,
}
#[event]
pub struct SetAdminEvent {

    pub admin_address: String,
    pub timestamp: i64,
}


#[event]
pub struct ChangeOperatorWalletEvent {
    pub admin: Pubkey,
    pub operator_wallet: Pubkey,
    pub time: i64,
}