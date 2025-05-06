use crate::constants::{DAY, USDC_DECIMALS};
use crate::error::PayContractsError;
use crate::event::MoneyUsed;
use crate::state::{
    BorrowAppl, EpochTime, InitializeAccount, PayMoneyDist, StakingVault, TreasuryAccount,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct PayMoney<'info> {
    #[account(constraint = fraud_agent.key() == initialize_acc.fraud_agent)]
    pub fraud_agent: Signer<'info>,

    #[account(mut, constraint = card_agent.key() == initialize_acc.card_provider)]
    pub card_agent: Signer<'info>,

    #[account(
        init_if_needed,
        payer = card_agent,
        associated_token::mint = usdc_mint,
        associated_token::authority = card_agent,
        associated_token::token_program = token_program,
    )]
    pub card_agent_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub borrow_appl: Account<'info, BorrowAppl>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury_acc.bump
    )]
    pub treasury_acc: Account<'info, TreasuryAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury_acc,
        associated_token::token_program = token_program,
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

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

    // #[account(address = USDC)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PayMoney>, req: Box<PayMoneyDist>) -> Result<()> {
    let borrow_appl = &mut ctx.accounts.borrow_appl;
    let treasury_token_account = &mut ctx.accounts.treasury_token_account;
    let treasury_acc = &mut ctx.accounts.treasury_acc;
    let usdc_mint = &ctx.accounts.usdc_mint;
    let card_agent_token_account = &mut ctx.accounts.card_agent_token_account;
    let current_timestamp = Clock::get()?.unix_timestamp;

    // Calculate total amount from the request struct
    let total_amount = req.req.iter().map(|inst| inst.amount).sum::<u64>();

    borrow_appl.used_amount += total_amount;

    require!(
        borrow_appl.used_amount <= borrow_appl.approved_amount,
        PayContractsError::NotEnoughFundsApproved
    );

    // Update staking vault values based on epoch time
    for inst in &req.req {
        match inst.epoch {
            EpochTime::FifteenDays => {
                if let Some(vault) = &mut ctx.accounts.fifteen_staking_vault {
                    require!(
                        vault.total_available >= inst.amount,
                        PayContractsError::InsufficientFunds
                    );
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

                    vault.total_available -= inst.amount;
                }
            }
            EpochTime::ThirtyDays => {
                if let Some(vault) = &mut ctx.accounts.thirty_staking_vault {
                    require!(
                        vault.total_available >= inst.amount,
                        PayContractsError::InsufficientFunds
                    );
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

                    vault.total_available -= inst.amount;
                }
            }
            EpochTime::FourtyFiveDays => {
                if let Some(vault) = &mut ctx.accounts.forty_five_staking_vault {
                    require!(
                        vault.total_available >= inst.amount,
                        PayContractsError::InsufficientFunds
                    );
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

                    vault.total_available -= inst.amount;
                }
            }
            EpochTime::SixtyDays => {
                if let Some(vault) = &mut ctx.accounts.sixty_staking_vault {
                    require!(
                        vault.total_available >= inst.amount,
                        PayContractsError::InsufficientFunds
                    );
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

                    vault.total_available -= inst.amount;
                }
            }
            EpochTime::SeventyFiveDays => {
                if let Some(vault) = &mut ctx.accounts.seventy_five_staking_vault {
                    require!(
                        vault.total_available >= inst.amount,
                        PayContractsError::InsufficientFunds
                    );
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

                    vault.total_available -= inst.amount;
                }
            }
            EpochTime::NinetyDays => {
                if let Some(vault) = &mut ctx.accounts.ninety_staking_vault {
                    require!(
                        vault.total_available >= inst.amount,
                        PayContractsError::InsufficientFunds
                    );
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

                    vault.total_available -= inst.amount;
                }
            }
        }
    }

    // Transfer funds from treasury to card agent
    let signer_seeds: &[&[&[u8]]] = &[&[b"treasury", &[treasury_acc.bump]]];

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: treasury_token_account.to_account_info(),
                to: card_agent_token_account.to_account_info(),
                mint: usdc_mint.to_account_info(),
                authority: treasury_acc.to_account_info(),
            },
            signer_seeds,
        ),
        total_amount,
        USDC_DECIMALS,
    )?;

    emit!(MoneyUsed {
        borrow_appl: borrow_appl.key(),
        money_dist: *req
    });

    Ok(())
}
