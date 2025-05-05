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
}
