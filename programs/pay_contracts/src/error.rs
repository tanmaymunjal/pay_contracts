use anchor_lang::error_code;

#[error_code]
pub enum PayContractsError {
    // 6000
    #[msg("Unauthorized Action")]
    UnauthorizedAction,

    // 6001
    #[msg("Approval can not be higher than requested amount")]
    ApprovalAmountTooHigh,

    // 6002
    #[msg("Epoch for this staking vault already started")]
    EpochAlreadyStarted,

    // 6003
    #[msg("Epoch has not ended yet")]
    EpochNotEnded,

    // 6004
    #[msg("Not enough approval")]
    NotEnoughFundsApproved,

    // 6005
    #[msg("Insufficient funds in staking vault selected")]
    InsufficientFunds,

    // 6006
    #[msg("Epoch not currently active")]
    EpochNotActive,

    // 6007
    #[msg("Epoch not selected correctly/invalid input")]
    InvalidEpochInput,

    // 6008
    #[msg("Can not return more than used")]
    CanNotReturnMoreThanUsed,
}
