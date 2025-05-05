use crate::constants::DAY;
use crate::error::PayContractsError;
use crate::event::AllocateBackMoneyEvent;
use crate::state::{EpochTime, InitializeAccount, PayMoneyDist, StakingVault, TreasuryAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AllocateBackMoney<'info> {
    #[account(constraint = fraud_agent.key() == initialize_acc.fraud_agent)]
    pub fraud_agent: Signer<'info>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury_acc.bump
    )]
    pub treasury_acc: Account<'info, TreasuryAccount>,
    #[account(
        seeds = [b"initialize"],
        bump = initialize_acc.bump
    )]
    pub initialize_acc: Account<'info, InitializeAccount>,

    // Add optional staking vaults
    #[account(mut)]
    pub fifteen_staking_vault: Option<Box<Account<'info, StakingVault>>>,

    #[account(mut)]
    pub thirty_staking_vault: Option<Box<Account<'info, StakingVault>>>,

    #[account(mut)]
    pub forty_five_staking_vault: Option<Box<Account<'info, StakingVault>>>,

    #[account(mut)]
    pub sixty_staking_vault: Option<Box<Account<'info, StakingVault>>>,

    #[account(mut)]
    pub seventy_five_staking_vault: Option<Box<Account<'info, StakingVault>>>,

    #[account(mut)]
    pub ninety_staking_vault: Option<Box<Account<'info, StakingVault>>>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AllocateBackMoney>, req: Box<PayMoneyDist>) -> Result<()> {
    let treasury_acc = &mut ctx.accounts.treasury_acc;
    let current_timestamp = Clock::get()?.unix_timestamp;

    // Calculate total amount from the request struct
    let total_amount = req.req.iter().map(|inst| inst.amount).sum::<u64>();

    require!(
        treasury_acc.available_funds >= total_amount,
        PayContractsError::NotEnoughAllocatableFunds
    );
    treasury_acc.available_funds -= total_amount;

    // Update staking vault values based on epoch time
    for inst in &req.req {
        match inst.epoch {
            EpochTime::FifteenDays => {
                if let Some(vault) = &mut ctx.accounts.fifteen_staking_vault {
                    // Calculate the epoch duration in seconds based on epoch_time
                    let epoch_duration = u64::from(vault.epoch_time) as i64 * DAY;
                    let epoch_end = vault.epoch_start + epoch_duration;
                    require!(
                        current_timestamp >= vault.epoch_start && current_timestamp <= epoch_end,
                        PayContractsError::EpochNotActive
                    );
                    require!(
                        vault.epoch_time == inst.epoch,
                        PayContractsError::InvalidEpochInput
                    );

                    vault.total_available += inst.amount;
                }
            }
            EpochTime::ThirtyDays => {
                if let Some(vault) = &mut ctx.accounts.thirty_staking_vault {
                    // Calculate the epoch duration in seconds based on epoch_time
                    let epoch_duration = u64::from(vault.epoch_time) as i64 * DAY;
                    let epoch_end = vault.epoch_start + epoch_duration;
                    require!(
                        current_timestamp >= vault.epoch_start && current_timestamp <= epoch_end,
                        PayContractsError::EpochNotActive
                    );
                    require!(
                        vault.epoch_time == inst.epoch,
                        PayContractsError::InvalidEpochInput
                    );

                    vault.total_available += inst.amount;
                }
            }
            EpochTime::FourtyFiveDays => {
                if let Some(vault) = &mut ctx.accounts.forty_five_staking_vault {
                    // Calculate the epoch duration in seconds based on epoch_time
                    let epoch_duration = u64::from(vault.epoch_time) as i64 * DAY;
                    let epoch_end = vault.epoch_start + epoch_duration;
                    require!(
                        current_timestamp >= vault.epoch_start && current_timestamp <= epoch_end,
                        PayContractsError::EpochNotActive
                    );
                    require!(
                        vault.epoch_time == inst.epoch,
                        PayContractsError::InvalidEpochInput
                    );

                    vault.total_available += inst.amount;
                }
            }
            EpochTime::SixtyDays => {
                if let Some(vault) = &mut ctx.accounts.sixty_staking_vault {
                    // Calculate the epoch duration in seconds based on epoch_time
                    let epoch_duration = u64::from(vault.epoch_time) as i64 * DAY;
                    let epoch_end = vault.epoch_start + epoch_duration;
                    require!(
                        current_timestamp >= vault.epoch_start && current_timestamp <= epoch_end,
                        PayContractsError::EpochNotActive
                    );
                    require!(
                        vault.epoch_time == inst.epoch,
                        PayContractsError::InvalidEpochInput
                    );

                    vault.total_available += inst.amount;
                }
            }
            EpochTime::SeventyFiveDays => {
                if let Some(vault) = &mut ctx.accounts.seventy_five_staking_vault {
                    // Calculate the epoch duration in seconds based on epoch_time
                    let epoch_duration = u64::from(vault.epoch_time) as i64 * DAY;
                    let epoch_end = vault.epoch_start + epoch_duration;
                    require!(
                        current_timestamp >= vault.epoch_start && current_timestamp <= epoch_end,
                        PayContractsError::EpochNotActive
                    );
                    require!(
                        vault.epoch_time == inst.epoch,
                        PayContractsError::InvalidEpochInput
                    );

                    vault.total_available += inst.amount;
                }
            }
            EpochTime::NinetyDays => {
                if let Some(vault) = &mut ctx.accounts.ninety_staking_vault {
                    // Calculate the epoch duration in seconds based on epoch_time
                    let epoch_duration = u64::from(vault.epoch_time) as i64 * DAY;
                    let epoch_end = vault.epoch_start + epoch_duration;
                    require!(
                        current_timestamp >= vault.epoch_start && current_timestamp <= epoch_end,
                        PayContractsError::EpochNotActive
                    );
                    require!(
                        vault.epoch_time == inst.epoch,
                        PayContractsError::InvalidEpochInput
                    );

                    vault.total_available += inst.amount;
                }
            }
        }
    }

    emit!(AllocateBackMoneyEvent {
        allocate_dist: *req
    });

    Ok(())
}
