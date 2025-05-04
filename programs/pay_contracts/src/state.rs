use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
#[repr(u8)]
pub enum EpochTime {
    FifteenDays,
    ThirtyDays,
    FourtyFiveDays,
    SixtyDays,
    SeventyFiveDays,
    NinetyDays,
}

#[account]
#[derive(InitSpace)]
pub struct InitializeAccount {
    pub bump: u8,
    pub card_provider: Pubkey,
    pub lending_agent: Pubkey,
    pub fraud_agent: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct TreasuryAccount {
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Borrower {
    pub bump: u8,
    pub total_borrowed_amount: u64,
    pub debarred: bool,
    pub collections: bool,
}

#[account]
#[derive(InitSpace)]
pub struct StakingVault {
    pub bump: u8,
    pub total_stake: u64,
    pub epoch_start: i64,
    pub epoch_time: EpochTime,
}

#[account]
#[derive(InitSpace)]
pub struct BorrowAppl {
    pub bump: u8,
    #[max_len(100)]
    pub appl_id: String,
    pub borrower: Pubkey,
    pub request_amount: u64,
    pub approved_amount: u64,
    #[max_len(100)]
    pub metadata: String,
}
