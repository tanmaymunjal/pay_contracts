use crate::constants::DAY;
use crate::event::StakingVaultCreated;
use crate::state::{EpochTime, StakingCenter, StakingVault};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed:String)]
pub struct CreateStakingVault<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"staking_center"
        ],
        bump=staking_center.bump
    )]
    pub staking_center: Account<'info, StakingCenter>,
    #[account(
        init,
        payer = initializer,
        space = 8 + StakingVault::INIT_SPACE,
        seeds = [
            b"staking_vault",
            seed.as_bytes()
        ],
        bump
    )]
    pub staking_vault: Account<'info, StakingVault>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateStakingVault>,
    seed: String,
    epoch_time: EpochTime,
) -> Result<()> {
    let staking_center = &mut ctx.accounts.staking_center;
    let staking_vault = &mut ctx.accounts.staking_vault;

    let current_timestamp = Clock::get()?.unix_timestamp;

    // Calculate the epoch duration in seconds based on epoch_time
    let epoch_duration = u64::from(epoch_time) as i64 * DAY;

    // Set initial vault properties
    staking_vault.bump = ctx.bumps.staking_vault;
    staking_vault.seed = seed;
    staking_vault.total_stake = 0;
    staking_vault.epoch_time = epoch_time;

    // Get the current expiry value for this epoch type
    let current_expiry = match epoch_time {
        EpochTime::FifteenDays => staking_center.fifteen_day_expiry,
        EpochTime::ThirtyDays => staking_center.thirty_day_expiry,
        EpochTime::FourtyFiveDays => staking_center.fourty_five_day_expiry,
        EpochTime::SixtyDays => staking_center.sixty_day_expiry,
        EpochTime::SeventyFiveDays => staking_center.seventy_five_day_expiry,
        EpochTime::NinetyDays => staking_center.ninety_day_expiry,
    };

    // Determine new epoch times based on current state
    let (epoch_start, new_expiry) =
        if current_expiry.is_none() || current_expiry.unwrap() <= current_timestamp {
            // Start a new epoch one day from now
            let start = current_timestamp + DAY;
            // Set the epoch end time
            let end = start + epoch_duration;
            (start, end)
        } else {
            // There's an existing active epoch
            let current_end = current_expiry.unwrap();
            // New vault's epoch starts when the current one ends
            let start = current_end;
            // Calculate the new epoch end time
            let end = current_end + epoch_duration;
            (start, end)
        };

    // Set the calculated epoch start for the vault
    staking_vault.epoch_start = epoch_start;

    // Update the appropriate staking center expiry field
    match epoch_time {
        EpochTime::FifteenDays => staking_center.fifteen_day_expiry = Some(new_expiry),
        EpochTime::ThirtyDays => staking_center.thirty_day_expiry = Some(new_expiry),
        EpochTime::FourtyFiveDays => staking_center.fourty_five_day_expiry = Some(new_expiry),
        EpochTime::SixtyDays => staking_center.sixty_day_expiry = Some(new_expiry),
        EpochTime::SeventyFiveDays => staking_center.seventy_five_day_expiry = Some(new_expiry),
        EpochTime::NinetyDays => staking_center.ninety_day_expiry = Some(new_expiry),
    };

    emit!(StakingVaultCreated {
        staking_vault: staking_vault.key(),
        epoch_start: epoch_start,
        epoch_time: epoch_time
    });

    Ok(())
}
