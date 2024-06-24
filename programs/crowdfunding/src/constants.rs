use anchor_lang::prelude::*;

#[constant]

pub const ADMIN_ROLE: &[u8] = b"ADMIN_ROLE";
pub const OPERATOR_ROLE: &[u8] = b"OPERATOR_ROLE";

    /// Seed for tran authority seed
pub const AUTHORITY_IDO: &[u8] = b"ido_pad";
pub const AUTHORITY_ADMIN: &[u8] = b"admin_ido";
pub const AUTHORITY_USER: &[u8] = b"wl_ido_pad";
pub const ONEPAD: &[u8] = b"onepad";