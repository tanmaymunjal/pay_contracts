use anchor_lang::prelude::*;
use instructions::*;

pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;

declare_id!("7Xx22mzpbLwcHkSMELjW2UEFsy2EzMU9yAjjm7vvzmRg");

#[program]
pub mod pay_contracts {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeContract>,
        card_provider: Pubkey,
        lending_agent: Pubkey,
        fraud_agent: Pubkey,
    ) -> Result<()> {
        initialize::handler(ctx, card_provider, lending_agent, fraud_agent)
    }

    pub fn edit_initialize(
        ctx: Context<EditContract>,
        card_provider: Pubkey,
        lending_agent: Pubkey,
        fraud_agent: Pubkey,
    ) -> Result<()> {
        edit_initialize::handler(ctx, card_provider, lending_agent, fraud_agent)
    }

    pub fn create_borrower(ctx: Context<CreateBorrower>) -> Result<()> {
        create_borrower::handler(ctx)
    }

    pub fn debarr_borrower(ctx: Context<DeBarrBorrower>, debarr: bool) -> Result<()> {
        debarr_borrower::handler(ctx, debarr)
    }

    pub fn collections_borrower(
        ctx: Context<CollectionsBorrower>,
        collections: bool,
    ) -> Result<()> {
        collections_borrower::handler(ctx, collections)
    }

    pub fn borrow_appl(
        ctx: Context<BorrowApplContx>,
        appl_id: String,
        request_amount: u64,
        metadata: String,
    ) -> Result<()> {
        borrow_appl::handler(ctx, appl_id, request_amount, metadata)
    }

    pub fn create_staking_vault(
        ctx: Context<CreateStakingVault>,
        seed: String,
        epoch_time: state::EpochTime,
    ) -> Result<()> {
        create_staking_vault::handler(ctx, seed, epoch_time)
    }

    pub fn stake_staking_vault(ctx: Context<StakeStakingVault>, stake_amount: u64) -> Result<()> {
        stake_staking_vault::handler(ctx, stake_amount)
    }

    pub fn unstake_staking_vault(ctx: Context<UnStakeStakingVault>) -> Result<()> {
        unstake_staking_vault::handler(ctx)
    }
}
