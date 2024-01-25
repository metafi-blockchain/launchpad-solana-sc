use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::AnchorDeserialize;
use anchor_lang::AnchorSerialize;
use solana_safe_math::SafeMath;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

static NATIVE_MINT: &str = "So11111111111111111111111111111111111111112";

declare_id!("6KMVQWmTXpd36ryMi7i91yeLsgM6S4BiaTX3UczEkvqq");

#[program]
pub mod crowdfunding {

    use std::{ops::Sub, str::FromStr};

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

        ido_account._raise_token = Pubkey::from_str(&raise_token).unwrap();
        ido_account._rate = rate;
        ido_account._open_timestamp = open_timestamp;

        ido_account._cap = cap;
        ido_account._closed = false;
        ido_account._owner = *ctx.accounts.user.key;

        ido_account._release_token = Pubkey::from_str(&release_token).unwrap();

        //add tier
        ido_account.add_tier(&TierItem {
            name: String::from("Lottery Winners"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account.add_tier(&TierItem {
            name: String::from("Top 100"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account.add_tier(&TierItem {
            name: String::from("Top 200"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account.add_tier(&TierItem {
            name: String::from("Top 300"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account.add_tier(&TierItem {
            name: String::from("Top 400"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account.add_tier(&TierItem {
            name: String::from("Top 500"),
            allocated: vec![],
            // allocated_count: 0,
        });
        ido_account.add_tier(&TierItem {
            name: String::from("Top 600"),
            allocated: vec![],
            // allocated_count: 0,
        });




        //check lai logic add round chỗ constructor của JD tier_allocations
        //add rounds
        ido_account.add_round(&RoundItem {
            name: String::from("Allocation"),
            duration_seconds: allocation_duration,
            class: RoundClass::Allocation,
            tier_allocations: vec![],
            participated: vec![],
        });

        ido_account.add_round(&RoundItem {
            name: String::from("FCFS - Prepare"),
            duration_seconds: 900,
            class: RoundClass::FcfsPrepare,
            tier_allocations: vec![],
            participated: vec![],
        });

        ido_account.add_round(&RoundItem {
            name: String::from("FCFS"),
            duration_seconds: fcfs_duration,
            class: RoundClass::Fcfs,
            tier_allocations: vec![],
            participated: vec![],
        });
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
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
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
        //delete round
        ido_account._rounds = vec![];

        //push round into ido_account._rounds
        for (i, name) in name_list.iter().enumerate() {
            ido_account.add_round(&RoundItem {
                name: name.to_string(),
                duration_seconds: duration_list[i],
                class: class_list[i].clone(),
                tier_allocations: vec![],
                participated: vec![],
            });
        }
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
        let user = &mut ctx.accounts.user;

        //check owner
        if ido_account._owner != *user.key {
            msg!("Invalid tiers specified");
            return Err(ProgramError::InvalidAccountOwner);
        }
        match ido_account._rounds.get_mut(index as usize) {
            Some(r) => {
                r.name = name;
                r.duration_seconds = duration_seconds;
                r.class = class;
            }
            None => {
                msg!("Invalid round index");
            }
        }
        Ok(())
    }

    pub fn modify_round_allocations(
        ctx: Context<ModifyTier>,
        index: u32,
        tier_allocations: Vec<u64>,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            msg!("Invalid tiers specified");
            return Err(ProgramError::InvalidAccountOwner);
        }

        match ido_account._rounds.get_mut(index as usize) {
            Some(r) => {
                r.tier_allocations = tier_allocations;
            }
            None => {
                msg!("Invalid round index");
            }
        }

        Ok(())
    }

    pub fn modify_tier(ctx: Context<Modifier>, index: u32, name: String) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }

        match ido_account._tiers.get_mut(index as usize) {
            Some(tier) => {
                tier.name = name;
            }
            None => {
                msg!("Invalid round index");
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
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        //delete tier
        ido_account._tiers = vec![];

        //push tier into ido_account._tiers
        for (_, name) in name_list.iter().enumerate() {
            ido_account.add_tier(&TierItem {
                name: name.to_string(),
                allocated: vec![],
                // allocated_count: 0,
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
        let user = &mut ctx.accounts.user;

        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }

        match ido_account._tiers.get_mut(index as usize) {
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
                msg!("Invalid round index");
            }
        }
        Ok(())
    }

    pub fn setup_release_token(
        ctx: Context<SetupReleaseToken>,
        token: String,
        pair: String,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        let token_pubkey = &Pubkey::from_str(&token).unwrap();
        let pair_pubkey = &Pubkey::from_str(&pair).unwrap();

        ido_account.set_release_token(token_pubkey, pair_pubkey );

        Ok(())
    }

    pub fn setup_releases(
        ctx: Context<SetupReleases>,
        from_timestamps: Vec<u32>,
        to_timestamps: Vec<u32>,
        percents: Vec<u16>,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;

        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }

        //check size
        if from_timestamps.len() != to_timestamps.len() || from_timestamps.len() != percents.len() {
            msg!("Invalid releases");
            return Err(ProgramError::InvalidArgument);
        }

        ido_account.set_releases(&from_timestamps, &to_timestamps, &percents) ;

        Ok(())
    }

    pub fn set_closed(ctx: Context<Modifier>, close: bool) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        ido_account.set_closed(&close);
        Ok(())
    }

    pub fn set_cap(ctx: Context<Modifier>, cap: u64) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        ido_account.set_cap(&cap);
        Ok(())
    }
    pub fn set_rate(ctx: Context<Modifier>, rate: u16) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        ido_account.set_rate(&rate);
        Ok(())
    }
    pub fn set_open_timestamp(ctx: Context<Modifier>, open_timestamp: u32)-> ProgramResult{
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        //check owner
        if ido_account._owner != *user.key{
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        ido_account.set_open_timestamp(&open_timestamp);
        Ok(())
        

    }

    //doing check lai ham nay la with draw balance cua SC ve vi ca nhan
    pub fn transfer_native_token(
        ctx: Context<TransferNativeToken>,
        amount: u64,
        to: Pubkey,
    ) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &mut ctx.accounts.user;
        // let system_program = &mut ctx.accounts.system_program;
        //check owner
        if ido_account._owner != *user.key {
            msg!("only authority is allowed to call this function");
            return Err(ProgramError::InvalidAccountOwner);
        }
        //transfer token
        let ix =
            anchor_lang::solana_program::system_instruction::transfer(&user.key(), &to, amount);
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.ido_info.to_account_info(),
            ],
        )?;

        Ok(())
    }

    //user join IDO
    pub fn participate(ctx: Context<Participate>, token: Pubkey, amount: u64) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        let user = &ctx.accounts.user;
        let system_program = &ctx.accounts.system_program;

        //check token is valid
        if ido_account._raise_token != token && check_id(&token) {
            msg!("{}", "Incorrect token specified");
            return Err(ProgramError::InvalidArgument);
        }
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

            //send sol to account ido address
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &user.key(),
                &ido_account.key(),
                amount,
            );
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[user.to_account_info(), ido_account.to_account_info()],
            )?;
        } else {
            //get amount token mint of user

            // transfer raise_token to account ido
            let transfer_instruction = spl_token::instruction::transfer(
                &ido_account._raise_token.key(),
                &user.key(),
                &ido_account.key(),
                &user.key(),
                &[],
                amount,
            )?;

            anchor_lang::solana_program::program::invoke_signed(
                &transfer_instruction,
                &[
                    user.to_account_info(),
                    ido_account.to_account_info(),
                    system_program.to_account_info(),
                ],
                &[&[&b"transfer"[..], &[0u8; 32]]],
            )?;
        }

        //emit event transfer
        emit!(ParticipateEvent {
            amount: amount,
            address: *user.key,
        });

        //update participated of contract
        ido_account._participated = ido_account._participated.safe_add(amount)?;

        if ido_account.get_participated_total(&user.key()) == 0 {
            ido_account._participated_count = ido_account._participated_count.add(1);
        }
        let sub_round = round.sub(1) as usize;

        match ido_account._rounds.get_mut(sub_round) {
            Some(mut _r) => {
                let _participated = _r
                    .get_participated_of_address(&user.key())
                    .safe_add(amount)?;
                //update participated of user
                _r.set_participated_of_address(&user.key(), &_participated);
            }
            None => {
                msg!("Invalid round index");
            }
        }

        Ok(())
    }

    //user claim token  : doing
    pub fn claim(ctx: Context<Claim>, index: u16, claimant: Pubkey) -> ProgramResult {
        let ido_account = &mut ctx.accounts.ido_info;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeIdoAccount<'info> {
    #[account(init, payer = user, space = 10000)]
    pub ido_info: Account<'info, IdoAccountInfo>,

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
    pub _owner: Pubkey,
    pub _release_token: Pubkey,
    pub _release_token_pair: Pubkey,
    pub _tiers: Vec<TierItem>,
    pub _rounds: Vec<RoundItem>,
    pub _releases: Vec<ReleaseItem>,
}
impl IdoAccountInfo {

    
    pub fn add_tier(&mut self, tier: &TierItem) {
        self._tiers.push(tier.clone());
    }
    pub fn add_round(&mut self, round: &RoundItem) {
        self._rounds.push(round.clone());
    }
    pub fn set_closed(&mut self, close: &bool){
        self._closed = close.clone();

    }
    pub fn set_cap(&mut self, cap: &u64){
        self._cap = cap.clone();

    }

    pub fn set_releases(&mut self, from_timestamps: &Vec<u32>, to_timestamps: &Vec<u32>, percents: &Vec<u16>){
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
    }

    pub fn set_release_token(&mut self,   token: &Pubkey, pair: &Pubkey ){
        self._release_token = token.clone();
        self._release_token_pair = pair.clone();
    }

    pub fn set_rate(&mut self, rate: &u16){
        self._rate = rate.clone();
    }

    pub fn set_open_timestamp(&mut self, open_timestamps: &u32){
        self._open_timestamp = open_timestamps.clone();
    }

    pub fn get_allocation(
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
                let raise_decimals = raise_token_decimals();
                let release_decimals = release_token_decimals();
    
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

    pub fn _info_wallet(&mut self, wallet: &Pubkey) -> (u16, u16, u8, String, u32) {
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
                                round_state_text = String::from("Allocation Round <u>closes</u> in:");
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
    
    pub fn close_timestamp(&self) -> u32 {
        let mut ts = self._open_timestamp;
        let rounds = self._rounds.clone();
        for (_, round) in rounds.iter().enumerate() {
            ts = ts.add(round.duration_seconds);
        }
        ts
    }
    
    pub fn fcfs_timestamp(&self) -> u32 {
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
        if self._closed
            || now_ts >= close_timestamp
            || self._participated >= self._cap
        {
            return true;
        }
    
        return false;
    }
    
   pub fn get_participated_total(&self, wallet: &Pubkey) -> u64 {
        let rounds = self._rounds.clone();
        let mut participated_total: u64 = 0;
        for (_, round) in rounds.iter().enumerate() {
            participated_total += round.get_participated_of_address(wallet);
        }
        return participated_total;
    }
    
    pub fn get_tier(&self, wallet: &Pubkey) -> u16 {
        let tiers = self._tiers.clone();
        for (i, tier) in tiers.iter().enumerate() {
            if tier.get_allocated(wallet) {
                return (i + 1) as u16;
            }
        }
        return 0;
    }
    
    pub fn get_allocation_remaining(&self, round: &u16, tier: &u16, wallet: &Pubkey, ) -> u64 {
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
    pub ido_info: Account<'info, IdoAccountInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

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

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
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

/**
 * Get info
 */



//doing
fn release_token_decimals() -> u8 {
    let mut decimals = 9;

    decimals
}
//doing
fn raise_token_decimals() -> u8 {
    let mut decimals = 9;

    decimals
}
//doing

