use anchor_lang::prelude::*;

use crate::AuthRole;

#[account]
pub struct AuthorityRole {
    pub bump: u8,         //1
    pub status: bool,    //1
    pub role: AuthRole, //4 + 4
    pub owner: Pubkey, //4 + 32 

}

impl AuthorityRole {

    pub fn initialize(
        &mut self,
        owner: &Pubkey,
        bump: u8,
        role: AuthRole,
    ) -> Result<()> {
        self.owner = *owner;
        self.bump = bump;
        self.status = true;
        self.role = role;
        Ok(())
    }
    
    pub fn set_status_account(&mut self, status: bool) {
        self.status = status;
    }

    pub fn set_role(&mut self, role: AuthRole) {
        self.role = role;
    }

    pub fn has_authority(&self, authority:Pubkey, role: AuthRole) -> bool {
        self.owner == authority && self.role == role
    }
}

 