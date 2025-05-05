use anchor_lang::error_code;

#[error_code]
pub enum PayContractsError {
    // 6000
    #[msg("Unauthorized Action")]
    UnauthorizedAction,

    // 6001
    #[msg("Approval can not be higher than requested amount")]
    ApprovalAmountTooHigh,
}
