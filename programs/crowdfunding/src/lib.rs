
mod instructions;
mod states;
mod utils;
mod errors;
mod events;

use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_safe_math::SafeMath;
use std::ops::Add;

declare_id!("A7HQd8NLQAj5DRxZUXS5vNkpUfDhnDRkHS8KhrP8eP1t");




#[program]
pub mod crowdfunding {

    use anchor_spl::associated_token::get_associated_token_address;
    use super::*;
    pub use instructions::*;
    pub use states::*;
    pub use utils::*;
    pub use errors::*;
    pub use events::*;

    /// Seed for tran authority seed
    pub const AUTHORITY_IDO: &[u8] = b"ido_pad";
    pub const AUTHORITY_ADMIN: &[u8] = b"admin_ido";
    pub const AUTHORITY_USER: &[u8] = b"wl_ido_pad";


    pub fn initialize(
        ctx: Context<InitializeIdoAccount>,
        raise_token: String,
        rate: u32,
        open_timestamp: i64,
        allocation_duration: u32,
        fcfs_duration: u32,
        cap: u64,
        release_token: String,
        ido_id: u64,
    ) -> Result<()> {

        let ido_account = &mut ctx.accounts.ido_account;
        let ido_admin_account   = &mut ctx.accounts.ido_admin_account;
        let token_mint = &ctx.accounts.token_mint;
        ido_admin_account._init_admin_ido(ctx.accounts.authority.key, &ido_account.key(), &ctx.bumps.ido_admin_account)?;

        ido_account.create_ido(
            &ido_admin_account.key(),
            &raise_token,
            &token_mint.decimals,
            &rate,
            &open_timestamp,
            &allocation_duration,
            &fcfs_duration,
            &cap,
            &release_token,
            &ido_id,
            &ctx.bumps.ido_account,
        )?;
        msg!("Create account success!");
        Ok(())
    }

    pub fn update_admin_ido( ctx: Context<UpdateAdminIdo>, admin_address : Pubkey)->Result<()>{
        let admin_account = &mut ctx.accounts.admin_wallet;
        admin_account._set_admin(&admin_address)?;
        Ok(())
    }

    pub fn modify_rounds(
        ctx: Context<AdminModifier>,
        name_list: Vec<String>,
        duration_list: Vec<u32>,
        class_list: Vec<RoundClass>
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;

        require!(name_list.len() > 0, IDOProgramErrors::InvalidRounds);
        require!(  name_list.len() == duration_list.len(), IDOProgramErrors::InvalidRounds);

        ido_account.modify_rounds(
            &name_list,
            &duration_list,
            &class_list
        )?;

        Ok(())
    }

    pub fn modify_round(
        ctx: Context<AdminModifier>,
        index: i32,
        name: String,
        duration_seconds: u32,
        class: RoundClass,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.modify_round(
            &index,
            &name,
            &duration_seconds,
            &class,
        )?;

        Ok(())
    }

    pub fn modify_round_allocations(
        ctx: Context<AdminModifier>,
        index: u8,
        tier_allocations: Vec<u64>,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;

        match ido_account._rounds.get_mut(index as usize) {
            Some(r) => {
                msg!("round {}", r.name);
               r.set_tier_allocation(tier_allocations)?;
            }
            None => {
                return err!(IDOProgramErrors::InvalidInDex);
            }
        }

        Ok(())
    }

    pub fn modify_tier(ctx: Context<AdminModifier>, index: u32, name: String) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;


        match ido_account._tiers.get_mut(index as usize) {
            Some(tier) => {
                tier.name = name;
            }
            None => {
                return err!(IDOProgramErrors::InvalidInDex);
            }
        }
        Ok(())
    }

    pub fn modify_tiers(ctx: Context<AdminModifier>, name_list: Vec<String>) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;

        require!(name_list.len() > 0, IDOProgramErrors::InValidTier);
        ido_account._tiers = vec![];
        //push tier into ido_account._tiers
        for (_, name) in name_list.iter().enumerate() {
            ido_account.add_tier(TierItem {
                name: name.to_string(),
                allocated_count: 0
            });
        }
        Ok(())
    }

    pub fn modify_tier_allocated_one(
        ctx: Context<ModifyTierAllocatedOne>,
        index: u8,
        address: Pubkey,
        remove: bool,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        let user_pda = &mut ctx.accounts.user_ido_account;

        //get data user pda
        if user_pda.bump != 0 && user_pda.address == address{
            user_pda.update_allocate(&index,  &!remove);
            ido_account.update_allocate_count( &(index as usize),  &!remove)?;

        }else {
            if !remove{
                user_pda.init_user_pda(&index, &address, &ido_account.key(), &!remove, &ctx.bumps.user_ido_account)?;
                ido_account.update_allocate_count( &(index as usize),  &!remove)?;
            }
        }
        
        Ok(())
    }

    pub fn setup_release_token(
        ctx: Context<SetupReleaseToken>,
        release_token: Pubkey,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        let token_mint: &Account<'_, Mint> = &ctx.accounts.token_mint;
        // let token_pubkey = &Pubkey::from_str(&token).unwrap();
        // let pair_pubkey = &Pubkey::from_str(&pair).unwrap();
        let decimals = token_mint.decimals;
        ido_account.set_release_token(
            &release_token,
            &decimals,
        )?;

        Ok(())
    }

    pub fn setup_releases(
        ctx: Context<AdminModifier>,
        from_timestamps: Vec<i64>,
        to_timestamps: Vec<i64>,
        percents: Vec<u16>,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        //check size
        require!( from_timestamps.len() == to_timestamps.len(), IDOProgramErrors::InvalidReleaseIndex);
        require!( to_timestamps.len() == percents.len(),  IDOProgramErrors::InvalidReleaseIndex);

        ido_account.set_releases(
            &from_timestamps,
            &to_timestamps,
            &percents,
        )?;

        Ok(())
    }

    pub fn set_closed(ctx: Context<AdminModifier>, close: bool) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_closed( &close)?;
        Ok(())
    }

    pub fn set_cap(ctx: Context<AdminModifier>, cap: u64) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_cap(&cap)?;
        Ok(())
    }

    pub fn set_rate(ctx: Context<AdminModifier>, rate: u32) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_rate( &rate)?;
        Ok(())
    }
    pub fn set_open_timestamp(ctx: Context<AdminModifier>, open_timestamp: i64) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_open_timestamp( &open_timestamp)?;
        Ok(())
    }

    // transferNativeToken
    // with draw token from pda of admin
    pub fn withdraw_native_token(
        ctx: Context<TransferNativeToken>,
        amount: u64,
        _to: Pubkey,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        let user = &ctx.accounts.authority;

        let rent_balance = Rent::get()?.minimum_balance(ido_account.to_account_info().data_len());
        let withdraw_amount = **ido_account.to_account_info().lamports.borrow() - rent_balance;

        require!(
            withdraw_amount >= amount,
            IDOProgramErrors::InsufficientAmount
        );

        **ido_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

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
        Ok(())
    }

    //user join IDO
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

    pub fn claim(ctx: Context<ClaimToken>, index: u16) -> Result<()> {
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
            let (_, _, _, _, _, _, remaining, status) = _get_allocation(&ido_account, &user_pda, ido_release_token_account, i as usize);
            
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
            msg!("claim success ");
            //emit ClaimEvent
            emit!(ClaimEvent {
                index: index,
                address: user_pda.address.to_string(),
                claim: remaining
            });
        }
        Ok(())
    }
  

}















