use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;


declare_id!("6KMVQWmTXpd36ryMi7i91yeLsgM6S4BiaTX3UczEkvqq");

#[program]
pub mod crowdfunding {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.total_donated = 0;
        campaign.admin = *ctx.accounts.user.key;
        // crowdfunding.total_amount = 0;
        // crowdfunding.total_donors = 0;

        // crowdfunding.state = State::Active;
        Ok(())
    } 

    pub fn withdraw(ctx: Context<Withdraw>, amount : u64) -> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        let user  = &mut ctx.accounts.user;
        if campaign.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId)
        }

        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount{
            return Err(ProgramError::InsufficientFunds);
        }

        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount : u64)-> ProgramResult{
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info()
            ]
        );
        (&mut ctx.accounts.campaign).total_donated += amount;

        Ok(())
    }


}

#[derive(Accounts)]
pub struct Create<'info>{
    #[account(init, payer = user, space = 9000, seeds=[b"CAMPAIGN_DEMO".as_ref(), user.key().as_ref()],bump)]
    pub campaign: Account<'info, CamPaign>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct CamPaign{
    pub name: String,
    pub description: String,
    pub total_donated: u64,
    pub admin: Pubkey,
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub campaign: Account<'info, CamPaign>,

    #[account(mut)]
    pub user: Signer<'info>,
   
}

#[derive(Accounts)]
pub struct Donate<'info>{
    #[account(mut)]
    pub campaign: Account<'info, CamPaign>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>
   
}