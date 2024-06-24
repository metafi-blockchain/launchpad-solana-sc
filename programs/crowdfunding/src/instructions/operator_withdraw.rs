
use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount, Mint};
use anchor_spl::associated_token::{get_associated_token_address, AssociatedToken};
use crate::{  AuthRole, AuthorityRole, IDOProgramErrors, IdoAccount, TokenTransferParams, WithdrawTokenEvent, _transfer_token_from_ido, };

use crate::{  AUTHORITY_IDO, OPERATOR_ROLE};
#[derive(Accounts)]
pub struct WithdrawTokenFromPda<'info> {
    #[account(mut,
        constraint = ido_account.authority == operator_pda.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_pda.bump,
        constraint = operator_pda.has_authority(authority.key(), AuthRole::Operator ) == true @ IDOProgramErrors::OnlyOperatorAllowed,
    )]
    pub operator_pda: Account<'info, AuthorityRole>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(init_if_needed,  
        payer = authority, 
        associated_token::mint = token_mint, 
        associated_token::authority = operator_wallet)]
    pub to_ata: Account<'info, TokenAccount>,
    ///CHECK:: only check operator wallet
    pub operator_wallet: AccountInfo<'info>,
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
        constraint = ido_account.authority == operator_pda.key(),
        seeds = [AUTHORITY_IDO, ido_account.ido_id.to_le_bytes().as_ref()], bump)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()],
        bump = operator_pda.bump,
        constraint = operator_pda.has_authority(authority.key(), AuthRole::Operator ) == true @ IDOProgramErrors::OnlyOperatorAllowed,
    )]
    pub operator_pda: Account<'info, AuthorityRole>,
    ///CHECK:: only check operator wallet
    pub operator_wallet: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_native_token(
    ctx: Context<TransferNativeToken>,
    amount: u64,
    _to: Pubkey,
) -> Result<()> {
    let ido_account = &mut ctx.accounts.ido_account;
    let operator_wallet = &mut ctx.accounts.operator_wallet;

    let rent_balance = Rent::get()?.minimum_balance(ido_account.to_account_info().data_len());
    let withdraw_amount = **ido_account.to_account_info().lamports.borrow() - rent_balance;

    require!(
        withdraw_amount >= amount,
        IDOProgramErrors::InsufficientAmount
    );

    **ido_account.to_account_info().try_borrow_mut_lamports()? -= amount;
    **operator_wallet.to_account_info().try_borrow_mut_lamports()? += amount;
    emit!(WithdrawTokenEvent{
        amount: amount,
        timestamp: Clock::get()?.unix_timestamp,
        address: operator_wallet.key().to_string(),
    });
    Ok(())
}

//transferToken
    //with draw token  only admin who create pda withdraw token
    pub fn withdraw_token_from_pda(ctx: Context<WithdrawTokenFromPda>, amount: u64) -> Result<()> {

        if !ctx.accounts.authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        
        let destination: &Account<'_, TokenAccount> = &mut ctx.accounts.to_ata;
        let ido_token_account = &mut ctx.accounts.from_ata;
        let token_program: &Program<'_, Token> = &ctx.accounts.token_program;
        let ido_account: &Account<'_, IdoAccount> = &ctx.accounts.ido_account;


        let _admin_token_address = get_associated_token_address(&ctx.accounts.authority.key(), &ido_account._raise_token);
        //require admin token account
        require!(_admin_token_address == destination.key(),  IDOProgramErrors::WithdrawTokenAccountNotMatch);

        let ido_id = ido_account.ido_id.to_le_bytes();
        let seeds: &[&[u8]] = &[AUTHORITY_IDO, ido_id.as_ref(), &[ctx.accounts.ido_account.bump]];
        let signer = &seeds[..];
        _transfer_token_from_ido( &TokenTransferParams {
            source: ido_token_account.to_account_info(),
            destination: destination.to_account_info(),
            authority: ido_account.to_account_info(),
            token_program: token_program.to_account_info(),
            authority_signer_seeds:signer,
            amount
        })?;
                //emit fn withdrawTokenEvent
        emit!(WithdrawTokenEvent{
            amount: amount,
            timestamp: Clock::get()?.unix_timestamp,
            address: destination.key().to_string(),
        });
        Ok(())





    }