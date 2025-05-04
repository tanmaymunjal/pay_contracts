use anchor_lang::prelude::*;
use crate::constants::DICTATOR;
use crate::error::PayContractsError;
use crate::state::InitializeAccount;
use crate::event::ContractInitialized;

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
    pub initialize_acc: Account<'info, InitializeAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeContract>, card_provider: Pubkey, lending_agent: Pubkey, fraud_agent: Pubkey
) -> Result<()>{
    let initialize_acc = &mut ctx.accounts.initialize_acc;

    initialize_acc.bump = ctx.bumps.initialize_acc;
    initialize_acc.card_provider = card_provider;
    initialize_acc.lending_agent = lending_agent;
    initialize_acc.fraud_agent = fraud_agent;

    emit!(ContractInitialized{
        card_provider,
        lending_agent,
        fraud_agent
    });

    Ok(())
}
