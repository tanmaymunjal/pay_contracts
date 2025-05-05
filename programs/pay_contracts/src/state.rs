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

impl From<EpochTime> for u64 {
    fn from(value: EpochTime) -> Self {
        match value {
            EpochTime::FifteenDays => 15,
            EpochTime::ThirtyDays => 30,
            EpochTime::FourtyFiveDays => 45,
            EpochTime::SixtyDays => 60,
            EpochTime::SeventyFiveDays => 75,
            EpochTime::NinetyDays => 90,
        }
    }
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
pub struct BorrowAppl {
    pub bump: u8,
    #[max_len(100)]
    pub appl_id: String,
    pub borrower: Pubkey,
    pub request_amount: u64,
    pub approved_amount: u64,
    pub used_amount: u64,
    #[max_len(100)]
    pub metadata: String,
}

#[account]
#[derive(InitSpace)]
pub struct StakingCenter {
    pub bump: u8,
    pub fifteen_day_expiry: Option<i64>,
    pub thirty_day_expiry: Option<i64>,
    pub fourty_five_day_expiry: Option<i64>,
    pub sixty_day_expiry: Option<i64>,
    pub seventy_five_day_expiry: Option<i64>,
    pub ninety_day_expiry: Option<i64>,
}

#[account]
#[derive(InitSpace)]
pub struct StakingVault {
    pub bump: u8,
    #[max_len(100)]
    pub seed: String,
    pub total_stake: u64,
    pub epoch_start: i64,
    pub epoch_time: EpochTime,
}
