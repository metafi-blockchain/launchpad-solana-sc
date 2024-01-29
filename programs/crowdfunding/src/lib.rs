use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;
use anchor_spl::{token::{TokenAccount, Mint,Token}};
use anchor_spl::token::Transfer;
use solana_safe_math::SafeMath;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

static NATIVE_MINT: &str = "So11111111111111111111111111111111111111112";

declare_id!("6KMVQWmTXpd36ryMi7i91yeLsgM6S4BiaTX3UczEkvqq");

#[program]
pub mod crowdfunding {

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
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let token_info = &ctx.accounts.token_info;
        ido_account.create_ido(
            ctx.accounts.user.key,
            &raise_token,
            &token_info.decimals,
            &rate,
            &open_timestamp,
            &allocation_duration,
            &fcfs_duration,
            &cap,
            &release_token,
        )?;
        msg!("Create account success!");
        Ok(())
    }

    //todo: check lại logic phan tiers khai bao
    pub fn modify_rounds(
        ctx: Context<Modifier>,
        name_list: Vec<String>,
        duration_list: Vec<u32>,
        class_list: Vec<RoundClass>,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;

        //check name_list
        if name_list.is_empty() {
            msg!("Invalid rounds specified");
            return Err(ProgramError::InvalidArgument);
        }
        //check size
        if name_list.len() != duration_list.len() || name_list.len() != class_list.len() {
            msg!("Invalid rounds specified");
            return Err(ProgramError::InvalidArgument);
        }

        ido_account.modify_rounds(
            ctx.accounts.user.key,
            &name_list,
            &duration_list,
            &class_list,
        )?;

        Ok(())
    }

    pub fn modify_round(
        ctx: Context<Modifier>,
        index: i32,
        name: String,
        duration_seconds: u32,
        class: RoundClass,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        ido_account.modify_round(
            ctx.accounts.user.key,
            &index,
            &name,
            &duration_seconds,
            &class,
        )?;

        Ok(())
    }

    pub fn modify_round_allocations(
        ctx: Context<ModifyTier>,
        index: u32,
        tier_allocations: Vec<u64>,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;

        //check owner
        if !ido_account._is_admin(ctx.accounts.user.key) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }

        match ido_account._rounds.get_mut(index as usize) {
            Some(r) => {
                r.tier_allocations = tier_allocations;
            }
            None => {
                msg!("Invalid round index");
                return Err(ProgramError::InvalidArgument);
            }
        }

        Ok(())
    }

    pub fn modify_tier(ctx: Context<Modifier>, index: u32, name: String) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;

        //check owner
        if !ido_account._is_admin(ctx.accounts.user.key) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }

        match ido_account._tiers.get_mut(index as usize) {
            Some(tier) => {
                tier.name = name;
            }
            None => {
                msg!("Invalid round index");
                return Err(ProgramError::InvalidArgument);
            }
        }
        Ok(())
    }

    pub fn modify_tiers(ctx: Context<Modifier>, name_list: Vec<String>) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;

        //check name_list
        if name_list.is_empty() {
            msg!("Invalid tiers specified");
            return Err(ProgramError::InvalidArgument);
        }
        //check owner
        if !ido_account._is_admin(user.key) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        //delete tier
        ido_account._tiers = vec![];

        //push tier into ido_account._tiers
        for (_, name) in name_list.iter().enumerate() {
            ido_account.add_tier(TierItem {
                name: name.to_string(),
                allocated: vec![],
            });
        }

        Ok(())
    }

    /**
     * them hoac remove address vao allocation cua tier
     */
    pub fn modify_tier_allocated(
        ctx: Context<Modifier>,
        index: u32,
        addresses: Vec<String>,
        remove: bool,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &ctx.accounts.user;

        //ido_account  implement modify_tier_allocated
        ido_account.modify_tier_allocated(user.key, &index, &addresses, &remove)?;
        Ok(())
    }

    pub fn setup_release_token(
        ctx: Context<SetupReleaseToken>,
        token: String,
        pair: String,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;

        let token_info: &Account<'_, Mint> =&ctx.accounts.token_info;

        let token_pubkey = &Pubkey::from_str(&token).unwrap();
        let pair_pubkey = &Pubkey::from_str(&pair).unwrap();
        let decimals = token_info.decimals;
        ido_account.set_release_token(ctx.accounts.user.key, token_pubkey, pair_pubkey , &decimals)?;

        Ok(())
    }

    pub fn setup_releases(
        ctx: Context<SetupReleases>,
        from_timestamps: Vec<u32>,
        to_timestamps: Vec<u32>,
        percents: Vec<u16>,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        //check size
        if from_timestamps.len() != to_timestamps.len() || from_timestamps.len() != percents.len() {
            msg!("Invalid releases");
            return Err(ProgramError::InvalidArgument);
        }

        ido_account.set_releases(
            ctx.accounts.user.key,
            &from_timestamps,
            &to_timestamps,
            &percents,
        )?;

        Ok(())
    }

    pub fn set_closed(ctx: Context<Modifier>, close: bool) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        ido_account.set_closed(ctx.accounts.user.key, &close)?;
        Ok(())
    }

    pub fn set_cap(ctx: Context<Modifier>, cap: u64) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        ido_account.set_cap(ctx.accounts.user.key, &cap)?;
        Ok(())
    }

    pub fn set_rate(ctx: Context<Modifier>, rate: u16) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        ido_account.set_rate(ctx.accounts.user.key, &rate)?;
        Ok(())
    }
    pub fn set_open_timestamp(ctx: Context<Modifier>, open_timestamp: u32) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        ido_account.set_open_timestamp(ctx.accounts.user.key, &open_timestamp)?;
        Ok(())
    }

    //doing check lai ham nay la with draw balance cua SC ve vi ca nhan
    pub fn transfer_native_token(
        ctx: Context<TransferNativeToken>,
        amount: u64,
        to: Pubkey,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &ctx.accounts.user;
        // let system_program = &mut ctx.accounts.system_program;
        //check owner
           if !ido_account._is_admin(user.key) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }

        let rent_balance = Rent::get()?.minimum_balance(ido_account.to_account_info().data_len());

        if **ido_account.to_account_info().lamports.borrow() - rent_balance < amount{
            return Err(ProgramError::InsufficientFunds);
        }

        **ido_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        // let ix = anchor_lang::solana_program::system_instruction::transfer(user.key, &to, amount);
        // anchor_lang::solana_program::program::invoke(
        //     &ix,
        //     &[
        //         ctx.accounts.user.to_account_info(),
        //         ctx.accounts.ido_info.to_account_info(),
        //     ],
        // )?;

        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferNativeToken>, token:Pubkey, amount: u64, to: Pubkey)->ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &ctx.accounts.user;
        let system_program = &mut ctx.accounts.system_program;
        //check owner
           if !ido_account._is_admin(user.key) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        //transfer spl token
        let transfer_instruction = spl_token::instruction::transfer(
            &token,
            &user.key(),
            &to,
            &user.key(),
            &[],
            amount,
        )?;
        //invoke_sign transfer token
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                user.to_account_info(),
                ido_account.to_account_info(),
                system_program.to_account_info(),
            ],
            &[&[&b"transfer"[..], &[0u8; 32]]],
        )?;
        
        Ok(())
    }
    //user join IDO: need test
    pub fn participate(ctx: Context<Participate>, amount: u64) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &ctx.accounts.user;
        let system_program = &ctx.accounts.system_program;


        //check amount
        if amount == 0 {
            msg!("{}", "Amount must be greater than 0");
            return Err(ProgramError::InvalidArgument);
        }

        let (tier, round, round_state, _, _) = ido_account._info_wallet(user.key);

        //check state
        if round_state != 1 && round_state != 3 {
            msg!("{}", "Participation not valid/open");
            return Err(ProgramError::ArithmeticOverflow); //
        }
        //check allocation remaining
        let allocation_remaining = ido_account.get_allocation_remaining(&round, &tier, user.key);
        if allocation_remaining < amount {
            msg!("{}", "Amount exceeds remaining allocation");
            return Err(ProgramError::InvalidArgument);
        }

        if ido_account._raise_token == Pubkey::from_str(NATIVE_MINT).unwrap() {
            //get user lam port
            let user_lamport = user.get_lamports();
            //check balance
            if user_lamport < amount {
                msg!("{}", "Insufficent native token balance");
                return Err(ProgramError::InvalidArgument);
            }

            let instruction =  anchor_lang::solana_program::system_instruction::transfer(
                &user.key(),
                &ido_account.key(),
                amount
            );
            anchor_lang::solana_program::program::invoke(
                &instruction,
                &[
                    user.to_account_info(),
                    ido_account.to_account_info(),
                ]
            )?;
        } else {
            //get amount token mint of user

            
        }

        //emit event transfer
        emit!(ParticipateEvent {
            amount: amount,
            address: *user.key,
        });

        //update participated of contract
        ido_account.update_participate(&round, user.key, &amount)?;

        Ok(())
    }

    pub fn test_participate(ctx: Context<Participate>, amount: u64)->ProgramResult{

        msg!("test participate");
        // let ido_account = &mut ctx.accounts.ido_info;

        let user = &ctx.accounts.user;


        let instruction =  anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.ido_info.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.ido_info.to_account_info(),
            ]
        )?;
        ctx.accounts.ido_info._participated_count += 1;
        // ido_account._participated_count.add(1);
        // transfer raise_token to account ido
        // let transfer_instruction = Transfer { 
        //     from : user.to_account_info(),
        //     to : ido_account.to_account_info(),
        //     authority: user.to_account_info()
        
        // };
        // let dep=&mut ctx.accounts.deposit_token_account.key();
        // let sender: &Signer<'_> = &ctx.accounts.user;
        // let inner=vec![sender.key.as_ref(),dep.as_ref(),"state".as_ref()];
        // let outer=vec![inner.as_slice()];
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.system_program.to_account_info(),
        //     transfer_instruction,
        //     outer.as_slice(),
        // );
        // anchor_spl::token::transfer(cpi_ctx, amount)?;


        //emit event transfer
        emit!(ParticipateEvent {
            amount: amount,
            address: *user.key,
        });

        // update participated of contract
        // ido_account.update_participate(&round, user.key, &amount)?;
        Ok(())
    }

    //user claim token  : doing
    pub fn claim(ctx: Context<Claim>, index: u16, claimant: Pubkey) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        //check release token
        if ido_account._release_token == Pubkey::from_str(NATIVE_MINT).unwrap() {
            msg!("Native token cannot be claimed");
            return Err(ProgramError::InvalidArgument);
        }
        //check index
        if index == 0 {
            msg!("Invalid release index");
            return Err(ProgramError::InvalidArgument);
        }
        //check claimant
        if claimant != *ctx.accounts.user.key {
            msg!("Invalid claimant");
            return Err(ProgramError::InvalidArgument);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeIdoAccount<'info> {
    #[account(init, payer = user, space = 10000)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account()]
    pub token_info: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct IdoAccountInfo {
    pub _raise_token: Pubkey,
    pub _rate: u16,
    pub _open_timestamp: u32,
    pub _cap: u64,
    pub _participated: u64,
    pub _participated_count: u32,
    pub _closed: bool,
    pub _release_token: Pubkey,
    pub _release_token_pair: Pubkey,
    pub _tiers: Vec<TierItem>,
    pub _rounds: Vec<RoundItem>,
    pub _releases: Vec<ReleaseItem>,
    _release_token_decimals: u8,
    _raise_token_decimals: u8,
    _owner: Pubkey,
}
trait IdoStrait {
    //setter function
    fn create_ido(
        &mut self,
        user: &Pubkey,
        raise_token: &String,
        decimals: &u8,
        rate: &u16,
        open_timestamp: &u32,
        allocation_duration: &u32,
        fcfs_duration: &u32,
        cap: &u64,
        release_token: &String,
    ) -> ProgramResult;

    fn init_tier(&mut self) -> ProgramResult;
    fn init_rounds(&mut self, allocation_duration: &u32, fcfs_duration: &u32) -> ProgramResult;
    //admin function
    fn add_tier(&mut self, tier: TierItem);
    fn add_round(&mut self, round: RoundItem);
    fn set_closed(&mut self, user: &Pubkey, close: &bool) -> ProgramResult;
    fn set_cap(&mut self, user: &Pubkey, cap: &u64) -> ProgramResult;

    fn set_releases(
        &mut self,
        user: &Pubkey,
        from_timestamps: &Vec<u32>,
        to_timestamps: &Vec<u32>,
        percents: &Vec<u16>,
    ) -> ProgramResult;

    fn set_release_token(&mut self, user: &Pubkey, token: &Pubkey, pair: &Pubkey, token_decimals: &u8) -> ProgramResult;

    fn modify_round(
        &mut self,
        user: &Pubkey,
        index: &i32,
        name: &String,
        duration_seconds: &u32,
        class: &RoundClass,
    ) -> ProgramResult;

    //check lai logic
    fn modify_rounds(
        &mut self,
        user: &Pubkey,
        name_list: &Vec<String>,
        duration_list: &Vec<u32>,
        class_list: &Vec<RoundClass>,
    ) -> ProgramResult;

    // modify_tier_allocated
    fn modify_tier_allocated(
        &mut self,
        user: &Pubkey,
        index: &u32,
        addresses: &Vec<String>,
        remove: &bool,
    ) -> Result<()>;

    fn set_rate(&mut self, user: &Pubkey, rate: &u16) -> ProgramResult;

    fn set_open_timestamp(&mut self, user: &Pubkey, open_timestamps: &u32) -> ProgramResult;

    //fn participate
    fn update_participate(&mut self, round: &u16, user: &Pubkey, amount: &u64) -> ProgramResult;

    // fn transfer_token( token: &Pubkey, from:&Pubkey,   to: &Pubkey,  amount: u64);
    // fn transfer_native_token( from:&Pubkey,   to: &Pubkey,  amount: u64);

    //claim
    fn _claim(&mut self, index: &u16, claimant: &Pubkey) -> ProgramResult;

    //getter function
    fn get_info_ido(&self) -> IdoAccountInfo;
    fn _is_admin(&self, user: &Pubkey) -> bool;

    fn _get_allocation(
        &mut self,
        wallet: &Pubkey,
        index: usize,
    ) -> (u32, u32, u16, u64, u64, u64, u64, u8);

    fn _info_wallet(&mut self, wallet: &Pubkey) -> (u16, u16, u8, String, u32);

    fn close_timestamp(&self) -> u32;

    fn fcfs_timestamp(&self) -> u32;

    fn _is_close(&self) -> bool;

    fn get_participated_total(&self, wallet: &Pubkey) -> u64;

    fn get_tier(&self, wallet: &Pubkey) -> u16;

    fn get_allocation_remaining(&self, round: &u16, tier: &u16, wallet: &Pubkey) -> u64;
}

impl IdoStrait for IdoAccountInfo {
    //implement create function
    fn create_ido(
        &mut self,
        user: &Pubkey,
        raise_token: &String,
        decimals: &u8,
        rate: &u16,
        open_timestamp: &u32,
        allocation_duration: &u32,
        fcfs_duration: &u32,
        cap: &u64,
        release_token: &String,
    ) -> ProgramResult {

        self._raise_token = Pubkey::from_str(raise_token).unwrap();
        self._raise_token_decimals = *decimals; 
        self._rate = *rate;
        self._open_timestamp = *open_timestamp;
        self._cap = *cap;
        self._closed = false;
        self._owner = *user;
        self._release_token = Pubkey::from_str(release_token).unwrap();
        self.init_tier()?;
        self.init_rounds(allocation_duration, fcfs_duration)?;
        Ok(())
    }

    fn init_tier(&mut self) -> ProgramResult {
        //add tier
        self.add_tier(TierItem {
            name: String::from("Lottery Winners"),
            allocated: vec![],
            // allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 100"),
            allocated: vec![],
            // allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 200"),
            allocated: vec![],
            // allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 300"),
            allocated: vec![],
            // allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 400"),
            allocated: vec![],
            // allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 500"),
            allocated: vec![],
            // allocated_count: 0,
        });
        self.add_tier(TierItem {
            name: String::from("Top 600"),
            allocated: vec![],
            // allocated_count: 0,
        });
        Ok(())
    }
    fn init_rounds(&mut self, allocation_duration: &u32, fcfs_duration: &u32) -> ProgramResult {
        //check lai logic add round chỗ constructor của JD tier_allocations
        //add rounds
        self.add_round(RoundItem {
            name: String::from("Allocation"),
            duration_seconds: *allocation_duration,
            class: RoundClass::Allocation,
            tier_allocations: vec![],
            participated: vec![],
        });

        self.add_round(RoundItem {
            name: String::from("FCFS - Prepare"),
            duration_seconds: 900,
            class: RoundClass::FcfsPrepare,
            tier_allocations: vec![],
            participated: vec![],
        });

        self.add_round(RoundItem {
            name: String::from("FCFS"),
            duration_seconds: *fcfs_duration,
            class: RoundClass::Fcfs,
            tier_allocations: vec![],
            participated: vec![],
        });

        Ok(())
    }

    fn add_tier(&mut self, tier: TierItem) {
        self._tiers.push(tier);
    }

    fn add_round(&mut self, round: RoundItem) {
        self._rounds.push(round);
    }

    fn set_closed(&mut self, user: &Pubkey, close: &bool) -> ProgramResult {
        if !self._is_admin(user) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        self._closed = *close;
        Ok(())
    }

    fn set_cap(&mut self, user: &Pubkey, cap: &u64) -> ProgramResult {
        if !self._is_admin(user) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        self._cap = *cap;
        Ok(())
    }

    fn set_releases(
        &mut self,
        user: &Pubkey,
        from_timestamps: &Vec<u32>,
        to_timestamps: &Vec<u32>,
        percents: &Vec<u16>,
    ) -> ProgramResult {
        if !self._is_admin(user) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        self._releases = vec![];
        //get info Ido from account address
        for (i, from_timestamp) in from_timestamps.iter().enumerate() {
            self._releases.push(ReleaseItem {
                from_timestamp: *from_timestamp,
                to_timestamp: to_timestamps[i],
                percent: percents[i],
                claimed: vec![],
            });
        }
        Ok(())
    }

    fn set_release_token(&mut self, user: &Pubkey, token: &Pubkey, pair: &Pubkey, token_decimals: &u8) -> ProgramResult {
        if !self._is_admin(user) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        self._release_token = *token;
        self._release_token_pair = *pair;

        //doing
        self._release_token_decimals = *token_decimals; //hardcode

        Ok(())
    }

    fn modify_round(
        &mut self,
        user: &Pubkey,
        index: &i32,
        name: &String,
        duration_seconds: &u32,
        class: &RoundClass,
    ) -> ProgramResult {
        if !self._is_admin(user) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }

        match self._rounds.get_mut(*index as usize) {
            Some(r) => {
                r.name = name.clone();
                r.duration_seconds = *duration_seconds;
                r.class = class.clone();
            }
            None => {
                msg!("Invalid round index");
                return Err(ProgramError::InvalidArgument);
            }
        }
        Ok(())
    }

    fn modify_rounds(
        &mut self,
        user: &Pubkey,
        name_list: &Vec<String>,
        duration_list: &Vec<u32>,
        class_list: &Vec<RoundClass>,
    ) -> ProgramResult {
        if !self._is_admin(user) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        self._rounds = vec![];

        //push round into ido_account._rounds
        for (i, name) in name_list.iter().enumerate() {
            self.add_round(RoundItem {
                name: name.to_string(),
                duration_seconds: duration_list[i],
                class: class_list[i].clone(),
                tier_allocations: vec![],
                participated: vec![],
            });
        }
        Ok(())
    }

    //modify_tier_allocated
    fn modify_tier_allocated(
        &mut self,
        user: &Pubkey,
        index: &u32,
        addresses: &Vec<String>,
        remove: &bool,
    ) -> Result<()> {
        if !self._is_admin(user) {
            return err!(ProgramErrors::PdaNotMatched);
        }

        match self._tiers.get_mut(*index as usize) {
            Some(tier) => {
                for (_, address) in addresses.iter().enumerate() {
                    let address = Pubkey::from_str(address).unwrap();
                    let al = {
                        &AllocateTier {
                            address,
                            allocated: !remove,
                        }
                    };
                    tier.add_allocated(al)
                }
            }
            None => {
                // msg!("Invalid round index");
                return err!(ProgramErrors::InvalidInDex);
            }
        }
        Ok(())
    }

    fn set_rate(&mut self, user: &Pubkey, rate: &u16) -> ProgramResult {
        if !self._is_admin(user) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        self._rate = *rate;
        Ok(())
    }

    fn set_open_timestamp(&mut self, user: &Pubkey, open_timestamps: &u32) -> ProgramResult {
        if !self._is_admin(user) {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        self._open_timestamp = open_timestamps.clone();
        Ok(())
    }

    //implement fn participate
    fn update_participate(&mut self, round: &u16, user: &Pubkey, amount: &u64) -> ProgramResult {
        //update participated of contract
        self._participated = self._participated.safe_add(*amount)?;

        if self.get_participated_total(user) == 0 {
            self._participated_count = self._participated_count.add(1);
        }
        let sub_round = round.sub(1) as usize;

        match self._rounds.get_mut(sub_round) {
            Some(mut _r) => {
                let _participated = _r.get_participated_of_address(user).safe_add(*amount)?;
                //update participated of user
                _r.set_participated_of_address(user, &_participated);
            }
            None => {
                msg!("Invalid round index");
                return Err(ProgramError::InvalidArgument);
            }
        }

        Ok(())
    }
    //fn transfer_Token
   

    //implement function _claim
    fn _claim(&mut self, index: &u16, claimant: &Pubkey) -> ProgramResult {
        let native_token_pub = Pubkey::from_str(NATIVE_MINT).unwrap();

        if self._release_token == native_token_pub {
            msg!("Release token not yet defined");
            return Err(ProgramError::InvalidArgument);
        }
        //emit ClaimEvent
        emit!(ClaimEvent {
            index: *index,
            address: *claimant,
            remaining: 0
        });
        Ok(())
    }

    fn _is_admin(&self, user: &Pubkey) -> bool {
        self._owner == *user
    }

    fn _get_allocation(
        &mut self,
        wallet: &Pubkey,
        index: usize,
    ) -> (u32, u32, u16, u64, u64, u64, u64, u8) {
        match self._releases.get(index) {
            Some(r) => {
                let _rate = self._rate;
                let mut remaining = 0;
                let percent = r.percent;
                let from_timestamp = r.from_timestamp;
                let to_timestamp = r.to_timestamp;
                let participated = self.get_participated_total(wallet);
                let raise_decimals = self._raise_token_decimals;
                let release_decimals = self._release_token_decimals;

                let mut total = participated
                    .safe_mul(_rate as u64)
                    .unwrap()
                    .safe_div(100000)
                    .unwrap()
                    .safe_mul(percent as u64)
                    .unwrap()
                    .safe_div(10000)
                    .unwrap();

                if raise_decimals > release_decimals {
                    let base = raise_decimals.sub(release_decimals);
                    total = total.safe_div(base.pow(10) as u64).unwrap();
                }

                if release_decimals > raise_decimals {
                    let base = release_decimals.sub(raise_decimals);
                    total = total.safe_mul(base.pow(10) as u64).unwrap();
                }

                let mut claimable = total;

                let now_ts = Clock::get().unwrap().unix_timestamp as u32;

                match to_timestamp > from_timestamp && now_ts < to_timestamp {
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

                let claimed = r.get_claimed_of_address(wallet);

                if claimed < claimable {
                    remaining = claimable.safe_sub(claimed).unwrap();
                }

                let mut status = 0;

                let native_token_pub = Pubkey::from_str(NATIVE_MINT).unwrap();
                // //check _release_token is equal publich key 1nc1nerator11111111111111111111111111111111
                if self._release_token == native_token_pub {
                    if from_timestamp == 0 || now_ts > from_timestamp {
                        status = 1;

                        //doing
                        // check balance _release_token
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

    fn _info_wallet(&mut self, wallet: &Pubkey) -> (u16, u16, u8, String, u32) {
        let mut round = 0;
        let mut round_state = 4;
        let mut round_state_text = String::from("");
        let mut round_timestamp = 0;
        let is_close = self._is_close();
        let tier = self.get_tier(wallet);
        if !is_close {
            let mut ts = self._open_timestamp;
            let now_ts = Clock::get().unwrap().unix_timestamp as u32;

            if now_ts < ts {
                round_state = 0;
                round_state_text = String::from("Allocation Round <u>opens</u> in:");
                round_timestamp = ts;
            } else {
                let rounds = self._rounds.clone();

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
            round as u16,
            round_state,
            round_state_text,
            round_timestamp,
        );
    }

    fn close_timestamp(&self) -> u32 {
        let mut ts = self._open_timestamp;
        let rounds = self._rounds.clone();
        for (_, round) in rounds.iter().enumerate() {
            ts = ts.add(round.duration_seconds);
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
                    ts = ts.safe_add(round.duration_seconds).unwrap();
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

    fn get_participated_total(&self, wallet: &Pubkey) -> u64 {
        let rounds = self._rounds.clone();
        let mut participated_total: u64 = 0;
        for (_, round) in rounds.iter().enumerate() {
            participated_total += round.get_participated_of_address(wallet);
        }
        return participated_total;
    }

    fn get_tier(&self, wallet: &Pubkey) -> u16 {
        let tiers = self._tiers.clone();
        for (i, tier) in tiers.iter().enumerate() {
            if tier.get_allocated(wallet) {
                return (i + 1) as u16;
            }
        }
        return 0;
    }

    fn get_allocation_remaining(&self, round: &u16, tier: &u16, wallet: &Pubkey) -> u64 {
        if *round == 0 || *tier == 0 {
            return 0;
        }
        let round_index = round.safe_sub(1).unwrap_or(0) as usize;
        let tier_index = tier.safe_sub(1).unwrap_or(0);
        let tiers = self._tiers.clone();
        let rounds = self._rounds.clone();

        match tiers.get(tier_index as usize) {
            Some(tier) => {
                if tier.get_allocated(wallet) {
                    match rounds.get(round_index) {
                        Some(round) => {
                            let participated = round.get_participated_of_address(wallet);
                            let allocated = round.get_tier_allocation(tier_index);
                            if participated < allocated {
                                return allocated.safe_sub(participated).unwrap();
                            }
                        }
                        None => {
                            return 0;
                        }
                    }
                }
            }
            None => {
                0;
            }
        }
        return 0;
    }

    fn get_info_ido(&self) -> IdoAccountInfo {
        self.clone()
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
    pub participated: Vec<Participated>,
}
impl RoundItem {
    pub fn get_participated_of_address(&self, address: &Pubkey) -> u64 {
        let participated = self.participated.clone();

        for (_, item) in participated.iter().enumerate() {
            if item.address == *address {
                return item.get_amount(address);
            }
        }
        return 0;
    }
    pub fn get_tier_allocation(&self, index: u16) -> u64 {
        let tier_allocations = self.tier_allocations.clone();
        match tier_allocations.get(index as usize) {
            Some(&tier) => {
                return tier;
            }
            None => {
                return 0;
            }
        }
    }
    pub fn set_participated_of_address(&mut self, address: &Pubkey, amount: &u64) {
        let participated = self.participated.clone();

        for (i, item) in participated.iter().enumerate() {
            if item.address == *address {
                self.participated[i].set_amount(address, *amount);
                break;
            }
        }
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
    claimed: Vec<ClaimedAmount>,
}
impl ReleaseItem {
    pub fn get_claimed_of_address(&self, address: &Pubkey) -> u64 {
        let claimed = self.claimed.clone();
        for (_, item) in claimed.iter().enumerate() {
            if item.address == *address {
                return item.amount;
            }
        }
        return 0;
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ClaimedAmount {
    pub address: Pubkey,
    pub amount: u64,
}

impl ClaimedAmount {
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
pub struct TierItem {
    pub name: String,
    pub allocated: Vec<AllocateTier>,
}

impl TierItem {
    pub fn add_allocated(&mut self, al: &AllocateTier) {
        let allocated = self.allocated.clone();
        //check al in allocated
        let mut check_exits = false;
        let mut index: usize = 0;
        for (i, item) in allocated.iter().enumerate() {
            if item.address == al.address {
                check_exits = true;
                index = i;
                break;
            }
        }
        //if exits update else add in to vector
        if check_exits {
            self.allocated[index].allocated = al.allocated;
        } else {
            self.allocated.push(al.clone());
        }
    }
    pub fn get_allocated(&self, address: &Pubkey) -> bool {
        let allocated = self.allocated.clone();

        for (_, item) in allocated.iter().enumerate() {
            if item.address == *address {
                return item.allocated;
            }
        }
        return false;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AllocateTier {
    pub address: Pubkey,
    pub allocated: bool,
}
impl AllocateTier {
    pub fn new(address: Pubkey, allocated: bool) -> Self {
        Self { address, allocated }
    }
    // check  allocated
    pub fn is_address_allocated(&self, address: Pubkey) -> bool {
        if self.address == address {
            return self.allocated;
        }
        return false;
    }
}

#[derive(Accounts)]
pub struct SetupReleaseToken<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account()]
    pub token_info: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyTier<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Participate<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    // #[account(mut)]
    // pub deposit_token_account:  Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    // pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetupReleases<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Modifier<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferNativeToken<'info> {
    #[account(mut)]
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/**
 * Get event structure
 */

#[event]
pub struct ParticipateEvent {
    pub amount: u64,
    pub address: Pubkey,
}
#[event]
pub struct ClaimEvent {
    pub index: u16,
    pub address: Pubkey,
    pub remaining: u64,
}

#[error_code]
pub enum ProgramErrors {
    #[msg("PDA account not matched")]
    PdaNotMatched,
    #[msg("Only authority is allowed to call this function")]
    NotAuthorized,
    #[msg("Invalid round index")]
    InvalidInDex,
    #[msg("Insufficient amount to withdraw.")]
    InvalidWithdrawAmount,

}
