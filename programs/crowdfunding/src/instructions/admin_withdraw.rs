
use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
use crate::{ AdminAccount, IdoAccount, AUTHORITY_ADMIN, AUTHORITY_IDO};


#[derive(Accounts)]
pub struct WithdrawTokenFromPda<'info> {
    #[account(mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account( has_one = authority,
        constraint = ido_account.key() == admin_wallet.owner,
        constraint = authority.key() == admin_wallet.authority,
        seeds = [AUTHORITY_ADMIN, ido_account.key().as_ref()], bump)]
    pub admin_wallet: Box<Account<'info, AdminAccount>>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(init_if_needed,  payer = authority, 
        associated_token::mint = token_mint, 
        associated_token::authority = authority)]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferNativeToken<'info> {
    #[account(mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account( has_one = authority, 
        constraint = ido_account.key() == admin_wallet.owner,
        constraint = authority.key() == admin_wallet.authority,
        seeds = [AUTHORITY_ADMIN, ido_account.key().as_ref()], bump)]
    pub admin_wallet: Account<'info, AdminAccount>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
