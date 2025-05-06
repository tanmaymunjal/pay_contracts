use crate::constants::{DAY, USDC_DECIMALS};
use crate::error::PayContractsError;
use crate::event::UnstakingEvenet;
use crate::state::{StakingVault, StakingVaultStake, TreasuryAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct UnStakeStakingVault<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = staker,
        associated_token::token_program = token_program,
    )]
    pub staker_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub staking_vault: Account<'info, StakingVault>,
    #[account(
        mut,
        seeds = [
            b"treasury"
        ],
        bump=treasury_acc.bump
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
        mut,
        close = staker,
        seeds = [
            b"staking",
            staker.key().as_ref(),
            staking_vault.key().as_ref()
        ],
        bump=stake_staking_vault.bump
    )]
    pub stake_staking_vault: Account<'info, StakingVaultStake>,
    // #[account(address=USDC)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UnStakeStakingVault>) -> Result<()> {
    let treasury_acc = &mut ctx.accounts.treasury_acc;
    let staker = &ctx.accounts.staker;
    let staking_vault = &mut ctx.accounts.staking_vault;
    let stake_staking_vault = &mut ctx.accounts.stake_staking_vault;
    let staker_token_account = &mut ctx.accounts.staker_token_account;
    let treasury_token_account = &mut ctx.accounts.treasury_token_account;
    let usdc_mint = &ctx.accounts.usdc_mint;
    let current_timestamp = Clock::get()?.unix_timestamp;

    // Calculate the epoch duration in seconds based on epoch_time
    let epoch_duration = u64::from(staking_vault.epoch_time) as i64 * DAY;
    let epoch_end = staking_vault.epoch_start + epoch_duration;

    require!(
        current_timestamp > epoch_end,
        PayContractsError::EpochNotEnded
    );

    let transfer_amount = (stake_staking_vault.stake_amount * staking_vault.total_available)
        / (staking_vault.total_stake);

    let signer_seeds: &[&[&[u8]]] = &[&[b"treasury", &[treasury_acc.bump]]];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: treasury_token_account.to_account_info(),
                to: staker_token_account.to_account_info(),
                authority: treasury_acc.to_account_info(),
                mint: usdc_mint.to_account_info(),
            },
            signer_seeds,
        ),
        transfer_amount,
        USDC_DECIMALS,
    )?;

    emit!(UnstakingEvenet {
        staker: staker.key(),
        staking_vault: staking_vault.key(),
        staked_amount: stake_staking_vault.stake_amount,
        unstaked_amount: transfer_amount
    });

    Ok(())
}
