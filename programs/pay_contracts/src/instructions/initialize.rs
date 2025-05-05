use crate::constants::DICTATOR;
use crate::error::PayContractsError;
use crate::event::ContractInitialized;
use crate::state::{InitializeAccount, StakingCenter, TreasuryAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(
        mut,
        constraint = initializer.key() == DICTATOR @ PayContractsError::UnauthorizedAction
    )]
    pub initializer: Signer<'info>,
    #[account(
        init,
        payer=initializer,
        space=8+InitializeAccount::INIT_SPACE,
        seeds = [
            b"initialize"
        ],
        bump
    )]
    pub initialize_acc: Box<Account<'info, InitializeAccount>>,
    #[account(
        init,
        payer = initializer,
        space = 8 + TreasuryAccount::INIT_SPACE,
        seeds = [
            b"treasury"
        ],
        bump
    )]
    pub treasury_acc: Account<'info, TreasuryAccount>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury_acc,
        token::token_program = token_program,
    )]
    pub treasury_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        payer = initializer,
        space = 8 + StakingCenter::INIT_SPACE,
        seeds = [
            b"staking_center"
        ],
        bump
    )]
    pub staking_center: Box<Account<'info, StakingCenter>>,
    #[account(address=USDC)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeContract>,
    card_provider: Pubkey,
    lending_agent: Pubkey,
    fraud_agent: Pubkey,
) -> Result<()> {
    let initialize_acc = &mut ctx.accounts.initialize_acc;
    let treasury_acc = &mut ctx.accounts.treasury_acc;
    let staking_center = &mut ctx.accounts.staking_center;

    treasury_acc.bump = ctx.bumps.treasury_acc;
    staking_center.bump = ctx.bumps.staking_center;

    initialize_acc.bump = ctx.bumps.initialize_acc;
    initialize_acc.card_provider = card_provider;
    initialize_acc.lending_agent = lending_agent;
    initialize_acc.fraud_agent = fraud_agent;

    emit!(ContractInitialized {
        card_provider,
        lending_agent,
        fraud_agent
    });

    Ok(())
}
