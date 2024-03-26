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
     pub index: u16,
     pub address: String,
     pub claim: u64,
 }