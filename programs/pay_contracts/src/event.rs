use crate::state::{EpochTime, PayMoneyDist};
use anchor_lang::prelude::*;

#[event]
pub struct ContractInitialized {
    pub card_provider: Pubkey,
    pub lending_agent: Pubkey,
    pub fraud_agent: Pubkey,
}

#[event]
pub struct ContractEdited {
    pub card_provider: Pubkey,
    pub lending_agent: Pubkey,
    pub fraud_agent: Pubkey,
}

#[event]
pub struct BorrowerCreated {
    pub borrower: Pubkey,
}

#[event]
pub struct BorrowerDeBarred {
    pub borrower: Pubkey,
    pub debarred: bool,
}

#[event]
pub struct CollectionsBorrowerEvent {
    pub borrower: Pubkey,
    pub collections: bool,
}

#[event]
pub struct BorrowApplEvent {
    pub borrower: Pubkey,
    pub request_amount: u64,
    pub metadata: String,
    pub appl_id: String,
}

#[event]
pub struct ApproveApl {
    pub borrow_appl: Pubkey,
    pub approval_amount: u64,
}

#[event]
pub struct StakingVaultCreated {
    pub staking_vault: Pubkey,
    pub epoch_start: i64,
    pub epoch_time: EpochTime,
}

#[event]
pub struct StakeStakingEvent {
    pub staker: Pubkey,
    pub staking_vault: Pubkey,
    pub stake_amount: u64,
}

#[event]
pub struct UnstakingEvenet {
    pub staker: Pubkey,
    pub staking_vault: Pubkey,
    pub staked_amount: u64,
    pub unstaked_amount: u64,
}

#[event]
pub struct MoneyUsed {
    pub borrow_appl: Pubkey,
    pub money_dist: PayMoneyDist,
}
