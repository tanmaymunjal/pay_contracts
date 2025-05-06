use crate::constants::DICTATOR;
use crate::error::PayContractsError;
use crate::event::ContractEdited;
use crate::state::InitializeAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditContract<'info> {
    #[account(
        mut,
        constraint = editor.key() == DICTATOR @ PayContractsError::UnauthorizedAction
    )]
    pub editor: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"initialize"
        ],
        bump=initialize_acc.bump
    )]
    pub initialize_acc: Account<'info, InitializeAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<EditContract>,
    card_provider: Pubkey,
    lending_agent: Pubkey,
    fraud_agent: Pubkey,
) -> Result<()> {
    let initialize_acc = &mut ctx.accounts.initialize_acc;

    initialize_acc.card_provider = card_provider;
    initialize_acc.lending_agent = lending_agent;
    initialize_acc.fraud_agent = fraud_agent;

    emit!(ContractEdited {
        card_provider,
        lending_agent,
        fraud_agent
    });

    Ok(())
}
