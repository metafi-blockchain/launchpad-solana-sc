
use anchor_lang::prelude::*;

#[account]
pub struct AdminAccount{
    pub authority: Pubkey,
    pub bump: u8,
    pub owner: Pubkey,
}
impl  AdminAccount {

    pub fn _set_admin(&mut self, admin: &Pubkey)->Result<()>{
        self.authority =  *admin;
        Ok(())
    }

    pub fn _is_admin(&self, admin: &Pubkey)->bool{
        self.authority == *admin
    }
    
    pub fn _init_admin_ido (&mut self, admin: &Pubkey,  owner: &Pubkey, bump: &u8)->Result<()>{
        self.authority =  *admin;
        self.owner = *owner;
        self.bump=*bump;
        Ok(())
    }
}
