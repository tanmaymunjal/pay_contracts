use anchor_lang::prelude::*;
use instructions::*;

pub mod state;
pub mod instructions;
pub mod constants;
pub mod error;
pub mod event;

declare_id!("7Xx22mzpbLwcHkSMELjW2UEFsy2EzMU9yAjjm7vvzmRg");

#[program]
pub mod pay_contracts {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContract>, card_provider: Pubkey, lending_agent: Pubkey, fraud_agent: Pubkey) -> Result<()> {
        initialize::handler(ctx, card_provider, lending_agent, fraud_agent)

    }
}
