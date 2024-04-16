use std::ops::Add;

use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount};
use solana_safe_math::SafeMath;
use crate::{IDOProgramErrors, IdoAccount, ParticipateEvent, PdaUserStats, _info_wallet, get_allocation_remaining, AUTHORITY_IDO, AUTHORITY_USER};

#[derive(Accounts)]
pub struct Participate<'info> {
    #[account(mut, seeds = [AUTHORITY_IDO , ido_account.ido_id.to_le_bytes().as_ref()], bump = ido_account.bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,

    #[account(mut, 
        constraint = user_pda_account.allocated == true,
        constraint = user_pda_account.address == user.key(),
        seeds = [AUTHORITY_USER,ido_account.key().as_ref(), user.key().as_ref()], bump = user_pda_account.bump)]
    pub user_pda_account: Account<'info, PdaUserStats>,

    #[account(mut,
        constraint = user_token_account.owner == user.key() @IDOProgramErrors::UserTokenAccountNotMatch,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut,
        constraint = ido_account._raise_token == ido_token_account.mint,
        constraint = ido_token_account.owner == ido_account.key() @IDOProgramErrors::IDoTokenAccountNotMatch,
    )]
    pub ido_token_account: Account<'info, TokenAccount>,
    #[account(signer)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn participate(ctx: Context<Participate>, amount: u64) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    let user_pda = &mut ctx.accounts.user_pda_account;
    let user: &Signer<'_> = &ctx.accounts.user;

    require!(amount > 0, IDOProgramErrors::InvalidAmount);

    let (_, round, round_state, _, _) = _info_wallet(ido_account, user_pda);
    msg!("round_state: {}", round_state);

    require!( round_state == 1 || round_state == 3, IDOProgramErrors::ParticipationNotValid);

    let allocation_remaining = get_allocation_remaining(ido_account, user_pda, &round);
    msg!("allocation_remaining {}", allocation_remaining);

    //check allocation remaining
    require!( allocation_remaining >= amount, IDOProgramErrors::AmountExceedsRemainingAllocation);

    //if raise token is native token
    if ido_account._raise_token == Pubkey::default() {
        //get user lam port
        let user_lamport = user.get_lamports();
        //check balance

        require!(user_lamport >= amount, IDOProgramErrors::InsufficientAmount);

        let instruction = anchor_lang::solana_program::system_instruction::transfer(
            user.key,
            &ido_account.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[user.to_account_info(), ido_account.to_account_info()],
        )?;
    } else {
        
        let destination = &ctx.accounts.ido_token_account;
        let source = &ctx.accounts.user_token_account;
        let token_program = &ctx.accounts.token_program;
        let authority = &ctx.accounts.user;

        //check amount token of user
        require!(source.amount >= amount, IDOProgramErrors::InsufficientAmount);

        // Transfer tokens from uer to pda
        let cpi_accounts = anchor_spl::token::Transfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };

        let cpi_program = token_program.to_account_info();

        anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

       
        msg!("Transfer succeeded!");
    }

    //emit event transfer
    emit!(ParticipateEvent {
        amount: amount,
        address: user.key.to_string(),
    });

    //update participated of contract
    ido_account._participated = ido_account._participated.safe_add(amount)?;

    if user_pda.participate_amount == 0 {
       
        ido_account._participated_count  = ido_account._participated_count.add(1);
    }

    user_pda.user_update_participate(amount)?;

    Ok(())
}