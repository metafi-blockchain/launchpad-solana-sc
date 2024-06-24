use anchor_lang::prelude::*;

use crate::{ AuthorityRole, OnePad, AuthRole, IDOProgramErrors};
use crate::{ ADMIN_ROLE, ONEPAD, OPERATOR_ROLE};

#[derive(Accounts)]
#[instruction(new_operator: Pubkey)]
pub struct AddOperator<'info> {
    #[account(
        mut,
        seeds = [ONEPAD],
        bump = onepad_pda.bump,
        constraint = onepad_pda.has_admin(admin_pda.key()) @ IDOProgramErrors::OnlyAdminAllowed,
    )]
    pub onepad_pda: Box<Account<'info, OnePad>>,

    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_pda.bump,
        constraint = admin_pda.has_authority(authority.key(), AuthRole::Admin ) == true @ IDOProgramErrors::OnlyAdminAllowed,
        constraint = admin_pda.status == true @ IDOProgramErrors::OnlyAdminAllowed,
    )]
    pub admin_pda:  Account<'info, AuthorityRole>,
    #[account(
        init,
        payer = authority,
        space = 60,
        seeds = [OPERATOR_ROLE, new_operator.as_ref() ],
        bump,
    )]
    pub operator_pda:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handle_add_operator(ctx: Context<AddOperator>, new_operator: Pubkey) -> Result<()> {
    let onepad_pda = &mut ctx.accounts.onepad_pda;
    let operator_pda = &mut ctx.accounts.operator_pda;
    operator_pda.initialize(&new_operator,ctx.bumps.operator_pda , AuthRole::Operator)?;
    onepad_pda.set_operator(operator_pda.key())?;
    Ok(())
}



#[derive(Accounts)]
#[instruction(old_operator: Pubkey)]
pub struct RemoveOperator<'info> {
    #[account(
        mut,
        seeds = [ONEPAD],
        bump = onepad_pda.bump,
        constraint = onepad_pda.has_admin(admin_pda.key()) @ IDOProgramErrors::OnlyAdminAllowed,
    )]
    pub onepad_pda: Box<Account<'info, OnePad>>,

    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_pda.bump,
        constraint = admin_pda.has_authority(authority.key(), AuthRole::Admin ) == true @ IDOProgramErrors::OnlyAdminAllowed,
        constraint = admin_pda.status == true @ IDOProgramErrors::OnlyAdminAllowed,
    )]
    pub admin_pda:  Account<'info, AuthorityRole>,
    #[account(
        mut, close = authority,
        seeds = [OPERATOR_ROLE, old_operator.as_ref() ],
        bump = operator_pda.bump,
    )]
    pub operator_pda:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}


pub fn handle_remove_operator(ctx: Context<RemoveOperator>, old_operator: Pubkey) -> Result<()> {
    
    let onepad_pda = &mut ctx.accounts.onepad_pda;
    let operator_pda = &mut ctx.accounts.operator_pda;
    require!(onepad_pda.has_operator(operator_pda.key()), IDOProgramErrors::OnlyOperatorAllowed);

    require!(operator_pda.has_authority(old_operator, AuthRole::Operator), IDOProgramErrors::OperatorNotFound);

    onepad_pda.remove_operator(operator_pda.key())?;
    Ok(())
}

