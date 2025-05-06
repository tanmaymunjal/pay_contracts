use crate::constants::DICTATOR;
use crate::error::PayContractsError;
use crate::event::BorrowerDeBarred;
use crate::state::Borrower;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeBarrBorrower<'info> {
    #[account(
        mut,
        // constraint = debarrer.key() == DICTATOR @ PayContractsError::UnauthorizedAction
    )]
    pub debarrer: Signer<'info>,
    #[account(mut)]
    pub borrower_acc: Account<'info, Borrower>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeBarrBorrower>, debarr: bool) -> Result<()> {
    let borrower_acc = &mut ctx.accounts.borrower_acc;
    borrower_acc.debarred = debarr;

    emit!(BorrowerDeBarred {
        borrower: borrower_acc.key(),
        debarred: debarr
    });

    Ok(())
}
