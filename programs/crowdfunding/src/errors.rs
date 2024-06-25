use anchor_lang::prelude::*;


#[error_code]
pub enum IDOProgramErrors {
    #[msg("PDA account not matched")]
    PdaNotMatched,
    #[msg("Only authority is allowed to call this function")]
    NotAuthorized,

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
    IDoTokenAccountNotMatch,
    #[msg("User token account not match")]
    UserTokenAccountNotMatch,
    #[msg("Admin token account not match")]
    WithdrawTokenAccountNotMatch,
    #[msg("Release token account of user not match")]
    ReleaseTokenAccountNotMatch,
    #[msg("Cannot parse data to account")]
    CannotParseData,

    #[msg("Only Admin Allowed")]
    OnlyAdminAllowed,
    #[msg("Only Operator Allowed")]
    OnlyOperatorAllowed,
    #[msg("Operator Not Found")]
    OperatorNotFound,

    #[msg("Operator Already Exist")]
    OperatorAlreadyExist,

    #[msg("Admin Limit Reached")]
    AdminLimitReached,

    #[msg("Operator Limit Reached")]
    OperatorLimitReached,

    #[msg("Admin Already Exist")]
    AdminAlreadyExist,

    #[msg("Operator wallet same as new wallet")]
    OperatorWalletSameAsNewWallet,

    #[msg("Address Zero")]
    AddressZero,

}

impl From<IDOProgramErrors> for ProgramError {
    fn from(e: IDOProgramErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}