use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_safe_math::SafeMath;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;
static NATIVE_MINT: &str = "So11111111111111111111111111111111111111112";

declare_id!("6KMVQWmTXpd36ryMi7i91yeLsgM6S4BiaTX3UczEkvqq");

#[program]
pub mod crowdfunding {

    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
    use anchor_spl::associated_token::get_associated_token_address;

    use super::*;

    pub fn initialize(
        ctx: Context<InitializeIdoAccount>,
        raise_token: String,
        rate: u16,
        open_timestamp: u32,
        allocation_duration: u32,
        fcfs_duration: u32,
        cap: u64,
        release_token: String,
        _ido_id: u32,
    ) -> Result<()> {

        let ido_account = &mut ctx.accounts.ido_account;
        let admin_bum =  &ctx.accounts.ido_admin_account.clone().bum;
        let ido_admin_account   = &mut ctx.accounts.ido_admin_account;
        let token_mint = &ctx.accounts.token_mint;
        ido_admin_account._init_admin_ido(ctx.accounts.authority.key, &ido_account.key(), admin_bum)?;

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
            &_ido_id,
        )?;
        msg!("Create account success!");
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

        // require!(index as usize < ido_account._tiers.len(), IDOProgramErrors::InValidTier);

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
        user_pda.init_user_pda(&index, &address, &ido_account.key(), &!remove)?;   
        //update tier count
        match ido_account._tiers.get_mut(index as usize){
            Some(tier) =>{
                if !remove {
                    tier.allocated_count = tier.allocated_count.add(1);
                }else{
                    if tier.allocated_count > 0 {
                        tier.allocated_count =  tier.allocated_count.sub(1);
                    }
                }
            }
            None=>{return err!(IDOProgramErrors::InvalidInDex)}
        }
        Ok(())
    }

    // pub fn modify_tier_allocated_multi(
    //     ctx: Context<ModifyTierAllocatedMulti>,
    //     index: u32,
    //     addresses: Vec<String>,
    //     _remove: bool
    // ) -> Result<()> {

    //     // let ido_account = & ctx.accounts.ido_account;
    //     let authority = &ctx.accounts.authority;
    //     match addresses.get(index as usize) {
    //         Some(address) => {
               
    //         //    let user =  Pubkey::from_str(address).unwrap();

    //             let system_program =  &ctx.accounts.system_program;
    //             let program_id = &Pubkey::from_str("6KMVQWmTXpd36ryMi7i91yeLsgM6S4BiaTX3UczEkvqq").unwrap();
               
    //             let _pub_user = Pubkey::from_str(address);
    //             let _ido_id = &ctx.accounts.ido_account.ido_id;
              
    //             let lamports = Rent::default().minimum_balance(1);

    //             let space = 32 + 1 + 32 + 16 + 16; //PdaUserIdo

    //             let seeds:&[&[u8]] = &[  b"claim_ido_pad", &authority.key.as_ref(), &_ido_id.to_le_bytes(),  &[ctx.bumps.ido_account]];
                
    //             let (pda, _bump) = Pubkey::find_program_address(&[  "claim_ido_pad".as_ref()], program_id);
                
    //             let signers_seeds = &[&seeds[..]];
    //             let cpi_create_account = system_instruction::create_account(
    //                 authority.key, &pda.key(), lamports, space, program_id);

    //                invoke_signed(&cpi_create_account, &[
    //                 authority.to_account_info(), 
                    
    //                 system_program.to_account_info(),
    //             ], signers_seeds)?;
    //         }
    //         None => {
    //             return err!(IDOProgramErrors::InvalidInDex);
    //         }
    //     }
    //     Ok(())
    // }

    pub fn setup_release_token(
        ctx: Context<SetupReleaseToken>,
        token: String,
        pair: String,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;

        let token_mint: &Account<'_, Mint> = &ctx.accounts.token_mint;

        let token_pubkey = &Pubkey::from_str(&token).unwrap();
        let pair_pubkey = &Pubkey::from_str(&pair).unwrap();
        let decimals = token_mint.decimals;
        ido_account.set_release_token(
            token_pubkey,
            pair_pubkey,
            &decimals,
        )?;

        Ok(())
    }

    pub fn setup_releases(
        ctx: Context<AdminModifier>,
        from_timestamps: Vec<u32>,
        to_timestamps: Vec<u32>,
        percents: Vec<u16>,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        //check size
        require!(
            from_timestamps.len() == to_timestamps.len(),
            IDOProgramErrors::InvalidReleaseIndex
        );
        require!(
            to_timestamps.len() == percents.len(),
            IDOProgramErrors::InvalidReleaseIndex
        );

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

    pub fn set_rate(ctx: Context<AdminModifier>, rate: u16) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_rate( &rate)?;
        Ok(())
    }
    pub fn set_open_timestamp(ctx: Context<AdminModifier>, open_timestamp: u32) -> Result<()> {
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
        //add security check
        // check user is singer
        if !ctx.accounts.authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        
        let destination: &Account<'_, TokenAccount> = &ctx.accounts.to_ata;
        let ido_token_account = &ctx.accounts.from_ata;
        let token_program: &Program<'_, Token> = &ctx.accounts.token_program;
        let ido_account: &Account<'_, IdoAccount> = &ctx.accounts.ido_account;


        let _admin_token_address = get_associated_token_address(&ctx.accounts.authority.key(), &ido_account._raise_token);
        //require admin token account
        require!(_admin_token_address == destination.key(),  IDOProgramErrors::WithdrawTokenAccountNotMatch);



        // Transfer tokens from taker to initializer
        let transfer_instruction = anchor_spl::token::Transfer {
            from: ido_token_account.to_account_info(),
            to: destination.to_account_info(),
            authority: ido_account.to_account_info(),
        };

        let admin = &ctx.accounts.authority.key();
        let _ido_id = &ctx.accounts.ido_account.ido_id;

        let _signer: &[&[&[u8]]] = &[&[
            b"ido_pad",
            admin.as_ref(),
            &_ido_id.to_le_bytes(),
            &[ctx.bumps.ido_account],
        ]];

        let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_instruction)
            .with_signer(_signer);
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    //user join IDO
    pub fn participate(ctx: Context<Participate>, amount: u64) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        let user_pda = &mut ctx.accounts.user_pda_account;
        let user: &Signer<'_> = &ctx.accounts.user;

        let _ido_raise_token_account = get_associated_token_address(&ido_account.key(), &ido_account._raise_token);

        require!(_ido_raise_token_account == ctx.accounts.deposit_token_account.key(),  IDOProgramErrors::DepositTokenAccountNotMatch);

        require!(amount > 0, IDOProgramErrors::InvalidAmount);

        let (_, round, round_state, _, _) = _info_wallet(ido_account, user_pda);
        msg!("round_state: {}", round_state);

        require!( round_state == 1 || round_state == 3, IDOProgramErrors::ParticipationNotValid);

        let allocation_remaining = get_allocation_remaining(ido_account, user_pda, &round);
        msg!("allocation_remaining {}", allocation_remaining);

        //check allocation remaining
        require!( allocation_remaining >= amount, IDOProgramErrors::AmountExceedsRemainingAllocation);

        //if raise token is native token
        if ido_account._raise_token == Pubkey::from_str(NATIVE_MINT).unwrap() {
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
            


            let destination = &ctx.accounts.receive_token_account;
            let source = &ctx.accounts.deposit_token_account;
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
        let ido_release_token_account = &ctx.accounts.ido_token_account;
        
        let user_token_account = &ctx.accounts.user_token_account;

        let _user_token_address = get_associated_token_address(&ctx.accounts.user.key(), &ido_account._release_token);

        //check user token address
        require!(_user_token_address == user_token_account.key(), IDOProgramErrors::ReleaseTokenAccountNotMatch);

        if ido_account._release_token == Pubkey::from_str("11111111111111111111111111111111").unwrap() {
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
    
            let destination = &ctx.accounts.user_token_account;
            let source = &ctx.accounts.ido_token_account;
            let token_program = &ctx.accounts.token_program;
            let ido_account = &ctx.accounts.ido_account;
    
            // Transfer tokens from taker to initializer
            let transfer_instruction = anchor_spl::token::Transfer {
                from: source.to_account_info(),
                to: destination.to_account_info(),
                authority: ido_account.to_account_info(),
            };
            let admin = &ctx.accounts.ido_account.authority.key();
            let _ido_id = &ctx.accounts.ido_account.ido_id;
    
            let seeds: &[&[u8]] = &[
                b"ido_pad",
                admin.as_ref(),
                &_ido_id.to_le_bytes(),
                &[ctx.bumps.ido_account],
            ];
            let signer = &[&seeds[..]];
    
            let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_instruction)
                .with_signer(signer);
            anchor_spl::token::transfer(cpi_ctx, remaining)?;
        
            user_pda.user_update_claim(remaining)?;
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

#[derive(Accounts)]
#[instruction(
    raise_token: String,
    rate: u16,
    open_timestamp: u32,
    allocation_duration: u32,
    fcfs_duration: u32,
    cap: u64,
    release_token: String,
    _ido_id: u32)]
pub struct InitializeIdoAccount<'info> {
    #[account(init_if_needed,  
        payer = authority,  space = 10240 ,  seeds = [b"ido_pad",ido_admin_account.key().as_ref(),  &_ido_id.to_le_bytes()], bump)]
    pub ido_account:  Box<Account<'info, IdoAccount>>,
    #[account(init_if_needed,  payer = authority,  space = 8 + 65,  seeds = [b"admin_ido", system_program.key().as_ref(),  &_ido_id.to_le_bytes()], bump)]
    pub ido_admin_account: Account<'info, AdminAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(init_if_needed,  payer = authority, associated_token::mint = token_mint, associated_token::authority = ido_account)]
    pub token_account:Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    // pub program_id: UncheckedAccount<'info>,
}

#[account]
pub struct AdminAccount{
    pub authority: Pubkey,
    pub bum: u8,
    pub owner: Pubkey,
}
impl  AdminAccount {

    fn _set_admin(&mut self, admin: &Pubkey)->Result<()>{
        self.authority =  *admin;
        Ok(())
    }

    fn _is_admin(&self, admin: &Pubkey)->bool{
        return self.authority == *admin;
    }
    fn _init_admin_ido (&mut self, admin: &Pubkey,  owner: &Pubkey, bumb: &u8)->Result<()>{
        self.authority =  *admin;
        self.owner = *owner;
        self.bum=*bumb;
        Ok(())
    }
}

#[account]
pub struct IdoAccount {
    pub ido_id: u32,
    pub _rate: u16,
    pub _open_timestamp: u32,
    pub _cap: u64,
    pub _participated: u64,
    pub _participated_count: u32,
    pub _closed: bool,
    pub _release_token_decimals: u8,
    pub _raise_token_decimals: u8,
    pub bump: u8,
    pub _release_token: Pubkey,
    pub _release_token_pair: Pubkey,
    pub _raise_token: Pubkey,
    pub authority: Pubkey,
    pub _tiers: Vec<TierItem>,
    pub _rounds: Vec<RoundItem>,
    pub _releases: Vec<ReleaseItem>,
  
}

trait IdoStrait {
    //setter function
    fn create_ido(
        &mut self,
        admin: &Pubkey,
        raise_token: &String,
        decimals: &u8,
        rate: &u16,
        open_timestamp: &u32,
        allocation_duration: &u32,
        fcfs_duration: &u32,
        cap: &u64,
        release_token: &String,
        ido_id: &u32,
    ) -> Result<()>;

    fn init_tier(&mut self) -> Result<()>;
    fn init_rounds(&mut self, allocation_duration: &u32, fcfs_duration: &u32) -> Result<()>;
    //admin function
    fn add_tier(&mut self, tier: TierItem);
    fn add_round(&mut self, round: RoundItem);
    fn set_closed(&mut self, close: &bool) -> Result<()>;
    fn set_cap(&mut self, cap: &u64) -> Result<()>;
    fn set_releases( &mut self, from_timestamps: &Vec<u32>, to_timestamps: &Vec<u32>, percents: &Vec<u16>,) -> Result<()>;
    fn set_release_token( &mut self, token: &Pubkey, pair: &Pubkey, token_decimals: &u8,) -> Result<()>;
    fn modify_round( &mut self, index: &i32, name: &String, duration_seconds: &u32, class: &RoundClass,) -> Result<()>;
    fn modify_rounds(&mut self, name_list: &Vec<String>, duration_list: &Vec<u32>, class_list: &Vec<RoundClass>, ) -> Result<()>;
    fn set_rate(&mut self, rate: &u16) -> Result<()>;
    fn set_open_timestamp(&mut self, open_timestamps: &u32) -> Result<()>;
    fn close_timestamp(&self) -> u32;
    fn fcfs_timestamp(&self) -> u32;
    fn _is_close(&self) -> bool;
    fn bump(&self) -> u8 ;

    // fn get_participated_total(&self, wallet: &Pubkey) -> u64;


}

impl IdoStrait for IdoAccount {
    //implement create function
    fn create_ido(
        &mut self,
        admin: &Pubkey,
        raise_token: &String,
        decimals: &u8,
        rate: &u16,
        open_timestamp: &u32,
        allocation_duration: &u32,
        fcfs_duration: &u32,
        cap: &u64,
        release_token: &String,
        ido_id: &u32,
    ) -> Result<()> {
        self._raise_token = Pubkey::from_str(raise_token).unwrap();
        self._raise_token_decimals = *decimals;
        self._rate = *rate;
        self._open_timestamp = *open_timestamp;
        self._cap = *cap;
        self._closed = false;
        self.authority = *admin;
        self.ido_id = *ido_id;
        self._release_token = Pubkey::from_str(release_token).unwrap();
        // self._raise_token_account = Pubkey::from_str(raise_token_account).unwrap();
        self.init_tier()?;
        self.init_rounds(allocation_duration, fcfs_duration)?;
        Ok(())
    }

    fn init_tier(&mut self) -> Result<()> {
        self._tiers = vec![];
        //add tier
        self.add_tier(TierItem {
            name: String::from("Lottery Winners"),
            allocated_count: 0,

        });
        self.add_tier(TierItem {
            name: String::from("Top 100"),
            allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 200"),
            allocated_count: 0,
        });
        Ok(())
    }
    fn init_rounds(&mut self, allocation_duration: &u32, fcfs_duration: &u32) -> Result<()> {
        //check lai logic add round chỗ constructor của JD tier_allocations
        self._rounds = vec![];
        self.add_round(RoundItem {
            name: String::from("Allocation"),
            duration_seconds: *allocation_duration,
            class: RoundClass::Allocation,
            tier_allocations: vec![],
        });

        self.add_round(RoundItem {
            name: String::from("FCFS - Prepare"),
            duration_seconds: 900,
            class: RoundClass::FcfsPrepare,
            tier_allocations: vec![],

        });

        self.add_round(RoundItem {
            name: String::from("FCFS"),
            duration_seconds: *fcfs_duration,
            class: RoundClass::Fcfs,
            tier_allocations: vec![],
  
        });

        Ok(())
    }

    fn add_tier(&mut self, tier: TierItem) {
        
        self._tiers.push(tier);
    }

    fn add_round(&mut self, round: RoundItem) {
        self._rounds.push(round);
    }

    fn set_closed(&mut self, close: &bool) -> Result<()> {
        self._closed = *close;
        Ok(())
    }

    fn set_cap(&mut self, cap: &u64) -> Result<()> {
        self._cap = *cap;
        Ok(())
    }

    fn set_releases( &mut self, from_timestamps: &Vec<u32>, to_timestamps: &Vec<u32>, percents: &Vec<u16>,) -> Result<()> {

        self._releases = vec![];
        //get info Ido from account address
        for (i, from_timestamp) in from_timestamps.iter().enumerate() {
            self._releases.push(ReleaseItem {
                from_timestamp: *from_timestamp,
                to_timestamp: to_timestamps[i],
                percent: percents[i],
            });
        }
        Ok(())
    }

    fn set_release_token(
        &mut self,
        token: &Pubkey,
        pair: &Pubkey,
        token_decimals: &u8,
    ) -> Result<()> {
        self._release_token = *token;
        self._release_token_pair = *pair;
        self._release_token_decimals = *token_decimals; //hardcode
        Ok(())
    }

    fn modify_round(
        &mut self,
        index: &i32,
        name: &String,
        duration_seconds: &u32,
        class: &RoundClass,
    ) -> Result<()> {
        match self._rounds.get_mut(*index as usize) {
            Some(r) => {
                r.name = name.clone();
                r.duration_seconds = *duration_seconds;
                r.class = class.clone();
            }
            None => {
                return err!(IDOProgramErrors::InvalidInDex);
            }
        }
        Ok(())
    }

    fn modify_rounds(
        &mut self,
        name_list: &Vec<String>,
        duration_list: &Vec<u32>,
        class_list: &Vec<RoundClass>,
    ) -> Result<()> {
        self._rounds = vec![];
        //push round into ido_account._rounds
        for (i, name) in name_list.iter().enumerate() {
            self.add_round(RoundItem {
                name: name.to_string(),
                duration_seconds: duration_list[i],
                class: class_list[i].clone(),
                tier_allocations: vec![],
            });
        }
        Ok(())
    }

   

    fn set_rate(&mut self, rate: &u16) -> Result<()> {
        self._rate = *rate;
        Ok(())
    }

    fn set_open_timestamp(&mut self, open_timestamps: &u32) -> Result<()> {
        self._open_timestamp = open_timestamps.clone();
        Ok(())
    }



    fn close_timestamp(&self) -> u32 {
        let mut ts = self._open_timestamp;
        let rounds = self._rounds.clone();
        for (_, round) in rounds.iter().enumerate() {
            ts = ts.safe_add(round.duration_seconds).unwrap();
        }
        ts
    }

    fn fcfs_timestamp(&self) -> u32 {
        let mut ts = self._open_timestamp;
        let rounds = self._rounds.clone();
        for (_, round) in rounds.iter().enumerate() {
            match round.class {
                RoundClass::FcfsPrepare => {
                    return ts;
                }
                RoundClass::Fcfs => {
                    return ts;
                }
                _ => {
                    ts = ts.add(round.duration_seconds);
                }
            }
        }
        return ts;
    }

    fn _is_close(&self) -> bool {
        let close_timestamp = self.close_timestamp();
     
        //get block time stamp
        let now_ts = Clock::get().unwrap().unix_timestamp as u32;
        //check close time  and pr
        if self._closed || now_ts >= close_timestamp || self._participated >= self._cap {
            return true;
        }

        return false;
    }
    fn bump(&self) -> u8 {
        self.bump
    }

    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum RoundClass {
    Allocation,
    FcfsPrepare,
    Fcfs,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RoundItem {
    pub name: String,
    pub duration_seconds: u32,
    pub class: RoundClass,
    pub tier_allocations: Vec<u64>,
   
}

impl RoundItem {
    pub fn get_tier_allocation(&self, index: u8) -> u64 {
        let tier_allocations = self.tier_allocations.clone();
        match tier_allocations.get(index as usize) {
            Some(&al) => {
                return al;
            }
            None => {
                return 0;
            }
        }
    }
    pub fn set_tier_allocation(&mut self, tier_allocations: Vec<u64>)->Result<()> {
        self.tier_allocations = tier_allocations;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Participated {
    pub address: Pubkey,
    pub amount: u64,
}

impl Participated {
    pub fn get_amount(&self, address: &Pubkey) -> u64 {
        if self.address == *address {
            return self.amount;
        }
        return 0;
    }
    pub fn set_amount(&mut self, address: &Pubkey, amount: u64) {
        if self.address == *address {
            self.amount = amount;
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ReleaseItem {
    from_timestamp: u32,
    to_timestamp: u32,
    percent: u16,
    // claimed: Vec<ClaimedAmount>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TierItem {
    pub name: String,
    pub allocated_count: u16
}


#[derive(Accounts)]
pub struct SetupReleaseToken<'info> {
    #[account(mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [b"ido_pad", admin_wallet.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
    #[account( has_one = authority, seeds = [b"admin_ido", system_program.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub admin_wallet: Account<'info, AdminAccount>,
    #[account(init_if_needed,  payer = authority, associated_token::mint = token_mint, associated_token::authority = ido_account)]
    pub release_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}



#[derive(Accounts)]
pub struct Participate<'info> {
    #[account(mut, seeds = [b"ido_pad", ido_account.authority.key().as_ref() , &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,

    #[account(mut, seeds = [b"wl_ido_pad", user.key().as_ref(), ido_account.key().as_ref()], bump)]
    pub user_pda_account: Account<'info, PdaUserStats>,

    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub receive_token_account: Account<'info, TokenAccount>,

    #[account(signer)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimToken<'info> {
    #[account(init_if_needed,  payer = user, associated_token::mint = token_mint, associated_token::authority = user)]
    pub user_token_account: Account<'info, TokenAccount>,
   
    #[account(mut, seeds = [b"ido_pad", ido_account.authority.key().as_ref() , &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,

    #[account(mut)]
    pub ido_token_account: Account<'info, TokenAccount>,

    #[account(   mut, seeds = [b"wl_ido_pad", user.key().as_ref(), ido_account.key().as_ref()], bump)]
    pub user_pda_account: Account<'info, PdaUserStats>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct AdminModifier<'info> {
    #[account(mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [b"ido_pad", admin_wallet.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
    #[account( has_one = authority, seeds = [b"admin_ido", system_program.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub admin_wallet: Account<'info, AdminAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferNativeToken<'info> {
    #[account(mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [b"ido_pad", admin_wallet.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
    #[account( has_one = authority, seeds = [b"admin_ido", system_program.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub admin_wallet: Account<'info, AdminAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawTokenFromPda<'info> {
    #[account(mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [b"ido_pad", admin_wallet.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
    #[account( has_one = authority, seeds = [b"admin_ido", system_program.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub admin_wallet: Account<'info, AdminAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    // pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(init_if_needed,  payer = authority, associated_token::mint = token_mint, associated_token::authority = authority)]
    pub to_ata: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyTierAllocatedMulti<'info>{
    #[account(mut, seeds = [b"ido_pad", ido_account.authority.key().as_ref() , &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
#[instruction(
    index: u8,
    address: Pubkey,
    remove: bool)]
pub struct ModifyTierAllocatedOne<'info> {
    #[account( init_if_needed, payer = authority, space = 8+32+32+16+16+1+1, 
        seeds = [b"wl_ido_pad", address.as_ref(), ido_account.key().as_ref()], bump)]
    pub user_ido_account: Account<'info, PdaUserStats>,
    #[account(mut,
        constraint = ido_account.authority == admin_wallet.key(),
        seeds = [b"ido_pad", admin_wallet.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
    #[account( has_one = authority, seeds = [b"admin_ido", system_program.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub admin_wallet: Account<'info, AdminAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PdaUserStats {
    pub address: Pubkey, //16
    pub tier_index: u8, //1
    pub allocated: bool, //1
    pub participate_amount: u64, //16
    pub claim_amount: u64, //16
    pub owner: Pubkey,//32
    pub bum: u8, //1
}



impl PdaUserStats{
    pub fn init_user_pda(&mut self, tier_index: &u8,address:&Pubkey, owner:&Pubkey, allocated: &bool) -> Result<()>  {
        self.tier_index = *tier_index;
        self.address = *address;
        self.owner = *owner;
        self.tier_index = *tier_index;
        self.allocated = *allocated;
        Ok(())

    }
    pub fn update_allocate(&mut self,tier_index: &u8, allocated:&bool){
        self.tier_index = *tier_index;
        self.allocated = *allocated; 
    }
    pub fn user_update_participate(&mut self, participate_amount:u64)-> Result<()>{
        self.participate_amount =  self.participate_amount.safe_add(participate_amount).unwrap(); 
        Ok(())
    }
    pub fn user_update_claim(&mut self, claim_amount:u64)-> Result<()>{
       
        let amount = self.claim_amount.safe_add(claim_amount).unwrap();
        self.claim_amount = amount; 
        Ok(())
    }

}


/**
 * Get event structure
 */

#[event]
pub struct ParticipateEvent {
    pub amount: u64,
    pub address: String,
}
#[event]
pub struct ClaimEvent {
    pub index: u16,
    pub address: String,
    pub claim: u64,
}

#[error_code]
pub enum IDOProgramErrors {
    #[msg("PDA account not matched")]
    PdaNotMatched,
    #[msg("Only authority is allowed to call this function")]
    NotAuthorized,
    #[msg("Invalid round index")]
    InvalidInDex,
    #[msg("Invalid rounds specified")]
    InvalidRounds,
    #[msg("Insufficient amount to withdraw.")]
    InsufficientAmount,
    #[msg("Invalid tiers specified")]
    InValidTier,
    #[msg("Invalid release index")]
    InvalidReleaseIndex,
    #[msg("Release token not yet defined")]
    InvalidReleaseToken,
    #[msg("No tokens left in the pool")]
    NoTokensLeft,
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Participation not valid/open")]
    ParticipationNotValid,
    #[msg("Amount exceeds remaining allocation")]
    AmountExceedsRemainingAllocation,
    #[msg("IDO token account not match")]
    DepositTokenAccountNotMatch,
    #[msg("Admin token account not match")]
    WithdrawTokenAccountNotMatch,
    #[msg("Release token account of user not match")]
    ReleaseTokenAccountNotMatch
}

impl From<IDOProgramErrors> for ProgramError {
    fn from(e: IDOProgramErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}

fn _info_wallet( ido_account:&mut IdoAccount,  user_pda: &mut PdaUserStats) -> (u8, u8, u8, String, u32) {
    
    let mut round = 0;
    let mut round_state = 4;
    let mut round_state_text = String::from("");
    let mut round_timestamp = 0;
    let is_close =  ido_account._is_close();
    let tier: u8 = if user_pda.allocated  { user_pda.clone().tier_index } else { 0 };

    if !is_close {
        let mut ts = ido_account._open_timestamp;
        let now_ts = Clock::get().unwrap().unix_timestamp as u32;
        if now_ts < ts {
            round_state = 0;
            round_state_text = String::from("Allocation Round <u>opens</u> in:");
            round_timestamp = ts;
        } else {
            let rounds = ido_account._rounds.clone();

            for (i, _round) in rounds.iter().enumerate() {
                round = i.add(1);
                ts = ts.safe_add(_round.duration_seconds).unwrap();
                if now_ts < ts {
                    match _round.class {
                        RoundClass::Allocation => {
                            round_state = 1;
                            round_state_text =
                                String::from("Allocation Round <u>closes</u> in:");
                            round_timestamp = ts;
                        }
                        RoundClass::FcfsPrepare => {
                            round_state = 2;
                            round_state_text = String::from("FCFS Round <u>opens</u> in:");
                            round_timestamp = ts;
                        }
                        RoundClass::Fcfs => {
                            round_state = 3;
                            round_state_text = String::from("FCFS Round <u>closes</u> in:");
                            round_timestamp = ts;
                        }
                    }
                    break;
                }
            }
        }
    }

    return (
        tier,
        round.try_into().unwrap() ,
        round_state,
        round_state_text,
        round_timestamp,
    );
}

fn get_allocation_remaining(ido_account:&mut IdoAccount, user_pda: &PdaUserStats ,round: &u8 ) -> u64 {

    let tier =  user_pda.tier_index;
    msg!("tier user {} ",tier );
    if *round == 0 || tier == 0 {
        return 0;
    }
   

    let round_index = round.sub(1) as usize;
    let _tier_index = tier.sub(1);
    let rounds = ido_account._rounds.clone();
    

    if user_pda.allocated {
        match rounds.get(round_index) {
            Some(round) => {
                let participated = user_pda.participate_amount;
                let allocated = round.get_tier_allocation(_tier_index);
                if participated < allocated {
                    return allocated.safe_sub(participated).unwrap();
                }
            }
            None => {
                return 0;
            }
        }  
    }
    return 0;
}


pub fn _get_allocation(
    ido_account: &IdoAccount,
    user_pda: &PdaUserStats,
    release_token_account: &TokenAccount, 
    index: usize,
) -> (u32, u32, u16, u64, u64, u64, u64, u8) {
    match ido_account._releases.get(index) {
        Some(r) => {
            let _rate: u16 = ido_account._rate;
            let mut status: u8 = 0;
            let mut remaining: u64 = 0;
            let percent: u16 = r.percent;
            let from_timestamp: u32 = r.from_timestamp;
            let to_timestamp: u32 = r.to_timestamp;
            let participated: u64 = user_pda.participate_amount;
            let raise_decimals: u8 = ido_account._raise_token_decimals;
            let release_decimals: u8 = ido_account._release_token_decimals;
            msg!("participated: {}",participated);
            let mut total: u64 = participated
                .safe_mul(_rate as u64)
                .unwrap()
                .safe_div(1000000)
                .unwrap()
                .safe_mul(percent as u64)
                .unwrap()
                .safe_div(10000)
                .unwrap();
            msg!("total: {}",total);
            if raise_decimals > release_decimals {
                let base: u32 = 10;
                total = total.safe_div(base.safe_pow(raise_decimals.sub(release_decimals)as u32).unwrap() as u64).unwrap();
            }

            if release_decimals > raise_decimals {  
                let base: u32 = 10;
                total = total.safe_mul(base.safe_pow(release_decimals.sub(raise_decimals) as u32).unwrap() as u64).unwrap();
            }

            let mut claimable = total;
            msg!("claimable: {}",claimable);
            let now_ts = Clock::get().unwrap().unix_timestamp as u32;

            msg!("to_timestamp: {}",to_timestamp);
            msg!("from_timestamp: {}",from_timestamp);
            msg!("now_ts: {}",now_ts);


            match (to_timestamp > from_timestamp) && (now_ts < to_timestamp)  {
                true => {
                    let mut elapsed = 0;
                    if now_ts > from_timestamp {
                        elapsed = now_ts.safe_sub(from_timestamp).unwrap();
                    }
                    let duration = to_timestamp.safe_sub(from_timestamp).unwrap();
                    claimable = total
                        .safe_mul(elapsed as u64)
                        .unwrap()
                        .safe_div(duration as u64)
                        .unwrap();
                }
                false => (),
            }
          
            let claimed = user_pda.claim_amount;
            msg!("claimed: {}",claimed);
            if claimed < claimable {
                remaining = claimable.safe_sub(claimed).unwrap();
            }   
            msg!("remaining: {}",remaining);

            let native_token_pub = Pubkey::from_str(NATIVE_MINT).unwrap();
            // //check _release_token is equal publich key 1nc1nerator11111111111111111111111111111111
            if ido_account._release_token != native_token_pub {
                if from_timestamp == 0 || now_ts > from_timestamp {
                    status = 1;

                    //check balance release token account > 0
                    if release_token_account.amount == 0 {
                        status = 2;
                    }
                    //check balance release pair token account > 0  //doing
                    // if(remaining == 0 || remaining > IERC20(_releaseToken).balanceOf(address(this)))
                    //     status = 2;
                }
            }

            return (
                from_timestamp,
                to_timestamp,
                percent,
                claimable,
                total,
                claimed,
                remaining,
                status,
            );
        }
        None => {
            msg!("Invalid release index");
            return (0, 0, 0, 0, 0, 0, 0, 0);
        }
    }
}

