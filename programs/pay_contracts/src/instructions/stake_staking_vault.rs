use crate::constants::USDC_DECIMALS;
use crate::error::PayContractsError;
use crate::event::StakeStakingEvent;
use crate::state::{StakingVault, StakingVaultStake, TreasuryAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct StakeStakingVault<'info> {
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
        init_if_needed,
        payer = staker,
        space = 8 + StakingVaultStake::INIT_SPACE,
        seeds = [
            b"staking",
            staker.key().as_ref(),
            staking_vault.key().as_ref()
        ],
        bump
    )]
    pub stake_staking_vault: Account<'info, StakingVaultStake>,
    // #[account(address=USDC)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<StakeStakingVault>, stake_amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.staker;
    let staking_vault = &mut ctx.accounts.staking_vault;
    let stake_staking_vault = &mut ctx.accounts.stake_staking_vault;
    let staker_token_account = &mut ctx.accounts.staker_token_account;
    let treasury_token_account = &mut ctx.accounts.treasury_token_account;
    let usdc_mint = &ctx.accounts.usdc_mint;
    let current_timestamp = Clock::get()?.unix_timestamp;

    require!(
        current_timestamp < staking_vault.epoch_start,
        PayContractsError::EpochAlreadyStarted
    );

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: staker_token_account.to_account_info(),
                to: treasury_token_account.to_account_info(),
                authority: staker.to_account_info(),
                mint: usdc_mint.to_account_info(),
            },
        ),
        stake_amount,
        USDC_DECIMALS,
    )?;

    stake_staking_vault.bump = ctx.bumps.stake_staking_vault;
    stake_staking_vault.staker = staker.key();
    stake_staking_vault.stake_amount += stake_amount;
    staking_vault.total_stake += stake_amount;
    staking_vault.total_available += stake_amount;

    emit!(StakeStakingEvent {
        staker: staker.key(),
        staking_vault: stake_staking_vault.key(),
        stake_amount: stake_amount
    });

    Ok(())
}
