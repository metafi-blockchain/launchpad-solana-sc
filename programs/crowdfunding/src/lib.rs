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

    // use anchor_spl::token;

    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

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
        let token_mint = &ctx.accounts.token_mint;
        let token_account = &ctx.accounts.token_account.key();
        ido_account.create_ido(
            ctx.accounts.authority.key,
            &raise_token,
            &token_mint.decimals,
            &rate,
            &open_timestamp,
            &allocation_duration,
            &fcfs_duration,
            &cap,
            &release_token,
            &token_account,
            &_ido_id,
        )?;
        msg!("Create account success!");
        Ok(())
    }

    pub fn modify_rounds(
        ctx: Context<Modifier>,
        name_list: Vec<String>,
        duration_list: Vec<u32>,
        class_list: Vec<RoundClass>,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;

        require!(name_list.len() > 0, IDOProgramErrors::InvalidRounds);
        require!(
            name_list.len() == duration_list.len(),
            IDOProgramErrors::InvalidRounds
        );

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
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
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
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;

        //check owner
        require!(
            ido_account._is_admin(&ctx.accounts.user.key),
            IDOProgramErrors::NotAuthorized
        );

        match ido_account._rounds.get_mut(index as usize) {
            Some(r) => {
                r.tier_allocations = tier_allocations;
            }
            None => {
                return err!(IDOProgramErrors::InvalidInDex);
            }
        }

        Ok(())
    }

    pub fn modify_tier(ctx: Context<Modifier>, index: u32, name: String) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;

        //check owner
        require!(
            ido_account._is_admin(&ctx.accounts.user.key),
            IDOProgramErrors::NotAuthorized
        );

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

    pub fn modify_tiers(ctx: Context<Modifier>, name_list: Vec<String>) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;

        //check owner
        require!(
            ido_account._is_admin(&ctx.accounts.user.key),
            IDOProgramErrors::NotAuthorized
        );
        require!(name_list.len() > 0, IDOProgramErrors::InValidTier);
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
     * add or remove the address into tier allocation  
     */
    pub fn modify_tier_allocated(
        ctx: Context<Modifier>,
        index: u32,
        addresses: Vec<String>,
        remove: bool,
    ) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        let user = &ctx.accounts.user;

        //ido_account  implement modify_tier_allocated
        ido_account.modify_tier_allocated(user.key, &index, &addresses, &remove)?;
        Ok(())
    }

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
            ctx.accounts.user.key,
            token_pubkey,
            pair_pubkey,
            &decimals,
        )?;

        Ok(())
    }

    pub fn setup_releases(
        ctx: Context<SetupReleases>,
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
            ctx.accounts.user.key,
            &from_timestamps,
            &to_timestamps,
            &percents,
        )?;

        Ok(())
    }

    pub fn set_closed(ctx: Context<Modifier>, close: bool) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_closed(ctx.accounts.user.key, &close)?;
        Ok(())
    }

    pub fn set_cap(ctx: Context<Modifier>, cap: u64) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_cap(ctx.accounts.user.key, &cap)?;
        Ok(())
    }

    pub fn set_rate(ctx: Context<Modifier>, rate: u16) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_rate(ctx.accounts.user.key, &rate)?;
        Ok(())
    }
    pub fn set_open_timestamp(ctx: Context<Modifier>, open_timestamp: u32) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        ido_account.set_open_timestamp(ctx.accounts.user.key, &open_timestamp)?;
        Ok(())
    }
    //transferNativeToken
    //with draw token from pda of admin
    pub fn withdraw_native_token(
        ctx: Context<TransferNativeToken>,
        amount: u64,
        _to: Pubkey,
    ) -> Result<()> {

        // check user is singer
        if !ctx.accounts.authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        let ido_account = &mut ctx.accounts.ido_account;
        let authority = &ctx.accounts.authority;
        //check owner
        require!( ido_account._is_admin(&ctx.accounts.authority.key), IDOProgramErrors::NotAuthorized );

        let rent_balance = Rent::get()?.minimum_balance(ido_account.to_account_info().data_len());
        let withdraw_amount = **ido_account.to_account_info().lamports.borrow() - rent_balance;

        require!( withdraw_amount >= amount, IDOProgramErrors::InsufficientAmount);

        **ido_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **authority.to_account_info().try_borrow_mut_lamports()? += amount;

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

        let destination = &ctx.accounts.to_ata;
        let source = &ctx.accounts.from_ata;
        let token_program = &ctx.accounts.token_program;
        let ido_account = &ctx.accounts.ido_account;


        // Transfer tokens from taker to initializer
        let transfer_instruction = anchor_spl::token::Transfer {
            from: source.to_account_info(),
            to: destination.to_account_info(),
            authority: ido_account.to_account_info(),
        };

        let admin = &ctx.accounts.authority.key();
        let _ido_id = &ctx.accounts.ido_account.ido_id;

        let seeds: &[&[&[u8]]] = &[&[
            b"sol_ido_pad",
            admin.as_ref(),
            &_ido_id.to_le_bytes(),
            &[ctx.bumps.ido_account],
        ]];

        let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_instruction)
            .with_signer(seeds);
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    //user join IDO: need test
    pub fn participate(ctx: Context<Participate>, amount: u64) -> Result<()> {
        let ido_account = &mut ctx.accounts.ido_account;
        let user = &ctx.accounts.user;
        // let system_program = &ctx.accounts.system_program;

        require!(amount > 0, IDOProgramErrors::InvalidAmount);

        let (tier, round, round_state, _, _) = ido_account._info_wallet(user.key);

        require!(
            round_state == 1 || round_state == 3,
            IDOProgramErrors::ParticipationNotValid
        );

        let allocation_remaining = ido_account.get_allocation_remaining(&round, &tier, user.key);

        //check allocation remaining
        require!(
            allocation_remaining >= amount,
            IDOProgramErrors::AmountExceedsRemainingAllocation
        );

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
            require!(amount >= amount, IDOProgramErrors::InsufficientAmount);

            let destination = &ctx.accounts.receive_token_account;
            let source = &ctx.accounts.deposit_token_account;
            let token_program = &ctx.accounts.token_program;
            let authority = &ctx.accounts.user;

            // Transfer tokens from taker to initializer
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
            address: *user.key,
        });

        //update participated of contract
        ido_account.update_participate(&round, user.key, &amount)?;

        Ok(())
    }

    //user claim token  : doing
    pub fn claim(ctx: Context<ClaimToken>, index: u16, _: Pubkey) -> Result<()> {
        // let ido_account = &mut ctx.accounts.ido_account;

        //transfer spl token from ido to user

        // ido_account._claim(&index, &claimant)?;

        //doing
        let amount = 1 * LAMPORTS_PER_SOL;

        require!(index > 0, IDOProgramErrors::InvalidReleaseIndex);

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

        // let seeds: &[&[&[u8]]] = &[&[b"sol_ido_pad", admin.as_ref(), &_ido_id.to_le_bytes(), &[ctx.bumps.ido_account]]];

        let seeds: &[&[u8]] = &[
            b"sol_ido_pad",
            admin.as_ref(),
            &_ido_id.to_le_bytes(),
            &[ctx.bumps.ido_account],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_instruction)
            .with_signer(signer);
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        msg!("claim success ");
        Ok(())
    }

    //function test
    pub fn transfer_spl_token(ctx: Context<TransferSplToken>, amount: u64) -> Result<()> {
        if !ctx.accounts.payer.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        let _amount = &ctx.accounts.from_ata.amount;
        require!(amount >= amount, IDOProgramErrors::InsufficientAmount);

        let destination = &ctx.accounts.to_ata;
        let source = &ctx.accounts.from_ata;
        let token_program = &ctx.accounts.token_program;
        let authority = &ctx.accounts.payer;

        // Transfer tokens from taker to initializer
        let cpi_accounts = anchor_spl::token::Transfer {
            from: source.to_account_info(),
            to: destination.to_account_info(),
            authority: authority.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();

        anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        msg!("Transfer succeeded!");
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
    raise_token_account: Pubkey,
    _ido_id: u32)]
pub struct InitializeIdoAccount<'info> {
    #[account(init_if_needed, 
        has_one = authority, 
        payer = authority, 
        space = 9000, 
        seeds = [b"sol_ido_pad", 
        authority.key().as_ref() , 
        &_ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(init_if_needed,  payer = authority, associated_token::mint = token_mint, associated_token::authority = ido_account)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
pub struct IdoAccount {
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
    pub _raise_token_account: Pubkey,
    // __raise_associated_token: Pubkey,
    _release_token_decimals: u8,
    _raise_token_decimals: u8,
    authority: Pubkey,
    ido_id: u32,
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
        raise_token_account: &Pubkey,
        ido_id: &u32,
    ) -> Result<()>;

    fn init_tier(&mut self) -> Result<()>;
    fn init_rounds(&mut self, allocation_duration: &u32, fcfs_duration: &u32) -> Result<()>;
    //admin function
    fn add_tier(&mut self, tier: TierItem);
    fn add_round(&mut self, round: RoundItem);
    fn set_closed(&mut self, user: &Pubkey, close: &bool) -> Result<()>;
    fn set_cap(&mut self, user: &Pubkey, cap: &u64) -> Result<()>;

    fn set_releases(
        &mut self,
        user: &Pubkey,
        from_timestamps: &Vec<u32>,
        to_timestamps: &Vec<u32>,
        percents: &Vec<u16>,
    ) -> Result<()>;

    fn set_release_token(
        &mut self,
        user: &Pubkey,
        token: &Pubkey,
        pair: &Pubkey,
        token_decimals: &u8,
    ) -> Result<()>;

    fn modify_round(
        &mut self,
        user: &Pubkey,
        index: &i32,
        name: &String,
        duration_seconds: &u32,
        class: &RoundClass,
    ) -> Result<()>;

    //check lai logic
    fn modify_rounds(
        &mut self,
        user: &Pubkey,
        name_list: &Vec<String>,
        duration_list: &Vec<u32>,
        class_list: &Vec<RoundClass>,
    ) -> Result<()>;

    // modify_tier_allocated
    fn modify_tier_allocated(
        &mut self,
        user: &Pubkey,
        index: &u32,
        addresses: &Vec<String>,
        remove: &bool,
    ) -> Result<()>;

    fn set_rate(&mut self, user: &Pubkey, rate: &u16) -> Result<()>;

    fn set_open_timestamp(&mut self, user: &Pubkey, open_timestamps: &u32) -> Result<()>;

    //fn participate
    fn update_participate(&mut self, round: &u16, user: &Pubkey, amount: &u64) -> Result<()>;

    // fn transfer_token( token: &Pubkey, from:&Pubkey,   to: &Pubkey,  amount: u64);
    // fn transfer_native_token( from:&Pubkey,   to: &Pubkey,  amount: u64);

    //claim
    fn _claim(&mut self, index: &u16, claimant: &Pubkey) -> Result<()>;

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

impl IdoStrait for IdoAccount {
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
        raise_token_account: &Pubkey,
        ido_id: &u32,
    ) -> Result<()> {
        self._raise_token = Pubkey::from_str(raise_token).unwrap();
        self._raise_token_decimals = *decimals;
        self._rate = *rate;
        self._open_timestamp = *open_timestamp;
        self._cap = *cap;
        self._closed = false;
        self.authority = *user;
        self.ido_id = *ido_id;
        self._release_token = Pubkey::from_str(release_token).unwrap();
        self._raise_token_account = *raise_token_account;
        self.init_tier()?;
        self.init_rounds(allocation_duration, fcfs_duration)?;
        Ok(())
    }

    fn init_tier(&mut self) -> Result<()> {
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
    fn init_rounds(&mut self, allocation_duration: &u32, fcfs_duration: &u32) -> Result<()> {
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

    fn set_closed(&mut self, user: &Pubkey, close: &bool) -> Result<()> {
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);
        self._closed = *close;
        Ok(())
    }

    fn set_cap(&mut self, user: &Pubkey, cap: &u64) -> Result<()> {
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);

        self._cap = *cap;
        Ok(())
    }

    fn set_releases(
        &mut self,
        user: &Pubkey,
        from_timestamps: &Vec<u32>,
        to_timestamps: &Vec<u32>,
        percents: &Vec<u16>,
    ) -> Result<()> {
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);

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

    fn set_release_token(
        &mut self,
        user: &Pubkey,
        token: &Pubkey,
        pair: &Pubkey,
        token_decimals: &u8,
    ) -> Result<()> {
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);

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
    ) -> Result<()> {
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);

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
        user: &Pubkey,
        name_list: &Vec<String>,
        duration_list: &Vec<u32>,
        class_list: &Vec<RoundClass>,
    ) -> Result<()> {
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);

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
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);

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
                return err!(IDOProgramErrors::InvalidInDex);
            }
        }
        Ok(())
    }

    fn set_rate(&mut self, user: &Pubkey, rate: &u16) -> Result<()> {
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);

        self._rate = *rate;
        Ok(())
    }

    fn set_open_timestamp(&mut self, user: &Pubkey, open_timestamps: &u32) -> Result<()> {
        require!(self._is_admin(user), IDOProgramErrors::NotAuthorized);

        self._open_timestamp = open_timestamps.clone();
        Ok(())
    }

    //implement fn participate
    fn update_participate(&mut self, round: &u16, user: &Pubkey, amount: &u64) -> Result<()> {
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
                return err!(IDOProgramErrors::InvalidInDex);
            }
        }

        Ok(())
    }
    //fn transfer_spl_token

    //implement function _claim
    fn _claim(&mut self, index: &u16, claimant: &Pubkey) -> Result<()> {
        let native_token_pub = Pubkey::from_str(NATIVE_MINT).unwrap();

        if self._release_token == native_token_pub {
            return err!(IDOProgramErrors::InvalidReleaseToken);
        }

        if *index == 0 {
            return err!(IDOProgramErrors::InvalidReleaseIndex);
        }
        for i in 0..*index {
            let (_, _, _, _, _, _, remaining, status) = self._get_allocation(claimant, i as usize);

            if status != 1 {
                continue;
            }

            //transfer token to claimant
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
        self.authority == *user
    }

    fn _get_allocation(
        &mut self,
        wallet: &Pubkey,
        index: usize,
    ) -> (u32, u32, u16, u64, u64, u64, u64, u8) {
        match self._releases.get(index) {
            Some(r) => {
                let _rate = self._rate;
                let mut status = 0;
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

                let native_token_pub = Pubkey::from_str(NATIVE_MINT).unwrap();
                // //check _release_token is equal publich key 1nc1nerator11111111111111111111111111111111
                if self._release_token == native_token_pub {
                    if from_timestamp == 0 || now_ts > from_timestamp {
                        status = 1;

                        //doing
                        // check balance _release_token
                        // if(_releaseTokenPair != address(0) && IERC20(_releaseToken).balanceOf(_releaseTokenPair) == 0)
                        //     status = 2;

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
    pub ido_account: Account<'info, IdoAccount>,
    #[account(init_if_needed,  payer = user, associated_token::mint = token_mint, associated_token::authority = ido_account)]
    pub release_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct ModifyTier<'info> {
    #[account(mut)]
    pub ido_account: Account<'info, IdoAccount>,
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Participate<'info> {
    #[account()]
    pub ido_account: Account<'info, IdoAccount>,

    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub receive_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimToken<'info> {
    #[account(init_if_needed,  payer = user, associated_token::mint = token_mint, associated_token::authority = user)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"sol_ido_pad", ido_account.authority.key().as_ref() , &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
    #[account(mut)]
    pub ido_token_account: Account<'info, IdoAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct SetupReleases<'info> {
    #[account(mut)]
    pub ido_account: Account<'info, IdoAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Modifier<'info> {
    #[account(mut)]
    pub ido_account: Account<'info, IdoAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferNativeToken<'info> {
    #[account(mut)]
    pub ido_account: Account<'info, IdoAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSplToken<'info> {
    pub payer: Signer<'info>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawTokenFromPda<'info> {
    #[account(mut,
        has_one = authority, 

         seeds = [b"sol_ido_pad", ido_account.authority.key().as_ref(), &ido_account.ido_id.to_le_bytes()], bump)]
    pub ido_account: Account<'info, IdoAccount>,
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
pub struct CreateTokenAccount<'info> {
    #[account(
        init_if_needed,
        payer = payer, 
        associated_token::mint = mint, 
        associated_token::authority = payer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    ///CHECK
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
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
}

impl From<IDOProgramErrors> for ProgramError {
    fn from(e: IDOProgramErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}
