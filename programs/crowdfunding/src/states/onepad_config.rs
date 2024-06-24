use anchor_lang::prelude::*;

use crate::IDOProgramErrors;
#[account]
pub struct OnePad {
    pub bump: u8,    //1
    pub pause: bool,
    pub operator_wallet: Pubkey,
    pub admin_role: Vec<Pubkey> ,  //4 + 32*3 = 36
    pub operator_role: Vec<Pubkey> ,
}

impl OnePad {
    pub fn initialize(
        &mut self,
        admin_role_pda: &Pubkey,
        operator_wallet: &Pubkey,
        bump: u8,

    ) -> Result<()> {
        self.bump = bump;
        self.pause = false;
        self.admin_role.push(*admin_role_pda);
        self.operator_wallet = *operator_wallet;
        Ok(())
    }
    
    pub fn set_pause(&mut self, pause: bool) {
        self.pause = pause;
    }
    
    pub fn has_admin(&self, authority: Pubkey) -> bool {
        self.admin_role.contains(&authority)
    }

    pub fn set_admin(&mut self, authority: Pubkey)-> Result<()> {
        if self.admin_role.contains(&authority) {
            return Err(IDOProgramErrors::AdminAlreadyExist.into());
        }
        if self.admin_role.len() >= 3 {
            return Err(IDOProgramErrors::AdminLimitReached.into());
        }
        self.admin_role.push(authority);
        Ok(())
    }
    pub fn remove_admin(&mut self, authority: Pubkey)-> Result<()> {

        if self.admin_role.len() == 1 {
            return Err(IDOProgramErrors::AdminLimitReached.into());
        }

        self.admin_role.retain(|&x| x != authority);
        Ok(())
    }

    pub fn change_operator_wallet(&mut self, new_operator_wallet: Pubkey) -> Result<()>{
        self.operator_wallet = new_operator_wallet;
        Ok(())
    }



    pub fn set_operator(&mut self, operator: Pubkey)-> Result<()> {
        if self.operator_role.contains(&operator) {
            return Err(IDOProgramErrors::OperatorAlreadyExist.into());
        }
        if self.operator_role.len() >= 3 {
            return Err(IDOProgramErrors::OperatorLimitReached.into());
        }
        self.operator_role.push(operator);
        Ok(())
    }
    pub fn remove_operator(&mut self, operator: Pubkey)-> Result<()> {
        if self.operator_role.len() == 1 {
            return Err(IDOProgramErrors::OperatorLimitReached.into());
        }
        self.operator_role.retain(|&x| x != operator);
        Ok(())
    }
    pub fn has_operator(&self, operator: Pubkey) -> bool {
        self.operator_role.contains(&operator)
    }
    
}