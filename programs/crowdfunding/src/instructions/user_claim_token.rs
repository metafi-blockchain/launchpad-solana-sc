use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::{get_associated_token_address, AssociatedToken};
use crate::{ClaimEvent, IDOProgramErrors, IdoAccount, PdaUserStats, TokenTransferParams, _get_allocation, _transfer_token_from_ido, AUTHORITY_IDO, AUTHORITY_USER};

#[derive(Accounts)]
pub struct ClaimToken<'info> {

    #[account(init_if_needed,  
        payer = user, 
        associated_token::mint = token_mint, 
        associated_token::authority = user)]
    pub user_token_account: Account<'info, TokenAccount>,
   
    #[account(mut, 
     seeds = [AUTHORITY_IDO , ido_account.ido_id.to_le_bytes().as_ref()], 
     bump = ido_account.bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,

    #[account(mut,
        constraint = ido_account._release_token == ido_token_account.mint,
        constraint = ido_token_account.owner == ido_account.key()
    )]
    pub ido_token_account: Account<'info, TokenAccount>,

    #[account(mut, 
        realloc = user_pda_account.get_size() + 9,
        realloc::zero = false,
        realloc::payer = user,
        constraint = user_pda_account.allocated == true,
        constraint = user_pda_account.address == user.key(),
        seeds = [AUTHORITY_USER, ido_account.key().as_ref(), user.key().as_ref()], bump = user_pda_account.bump)]
    pub user_pda_account: Account<'info, PdaUserStats>,

    #[account(mut, signer)]
    pub user: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn claim(ctx: Context<ClaimToken>, index: u8) -> Result<()> {
    let ido_account = &ctx.accounts.ido_account;
    let user_pda = &mut ctx.accounts.user_pda_account;
    let ido_release_token_account = &mut ctx.accounts.ido_token_account;
    // let release_token_pool_account = &mut ctx.accounts.release_token_pool_account;
    
    let user_token_account = &ctx.accounts.user_token_account;

    let _user_token_address = get_associated_token_address(&ctx.accounts.user.key(), &ido_account._release_token);

    //check user token address
    require!(_user_token_address == user_token_account.key(), IDOProgramErrors::ReleaseTokenAccountNotMatch);

    if ido_account._release_token == Pubkey::default() {
        return err!(IDOProgramErrors::InvalidReleaseToken);
    }

    if index == 0 {
        return err!(IDOProgramErrors::InvalidReleaseIndex);
    }

    for i in 0..index {
        let (_, _, _, _, _, _, remaining, status) = _get_allocation(ido_account, user_pda, ido_release_token_account, i as usize);
        
        if status != 1 {
            continue;
        }
        //transfer release token from pda to user

        let ido_id = ido_account.ido_id.to_le_bytes();
        let seeds: &[&[u8]] = &[AUTHORITY_IDO, ido_id.as_ref(), &[ctx.accounts.ido_account.bump]];
        let signer = &seeds[..];
      
        _transfer_token_from_ido( &TokenTransferParams {
            source: ido_release_token_account.to_account_info(),
            destination: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            authority_signer_seeds:signer,
            amount: remaining,
        })?;

        user_pda.user_claim(i,remaining)?;

        //emit ClaimEvent
        emit!(ClaimEvent {
            index: index,
            address: user_pda.address.to_string(),
            claim: remaining,
            timestamp: Clock::get()?.unix_timestamp,
        });
    }
    Ok(())
}