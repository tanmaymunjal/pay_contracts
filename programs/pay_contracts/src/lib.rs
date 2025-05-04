use anchor_lang::prelude::*;

declare_id!("7Xx22mzpbLwcHkSMELjW2UEFsy2EzMU9yAjjm7vvzmRg");

#[program]
pub mod pay_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
