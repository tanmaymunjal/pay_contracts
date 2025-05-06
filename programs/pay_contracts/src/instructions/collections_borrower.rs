use crate::constants::DICTATOR;
use crate::error::PayContractsError;
use crate::event::CollectionsBorrowerEvent;
use crate::state::Borrower;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CollectionsBorrower<'info> {
    #[account(
        mut,
        // constraint = collector.key() == DICTATOR @ PayContractsError::UnauthorizedAction
    )]
    pub collector: Signer<'info>,
    #[account(mut)]
    pub borrower_acc: Account<'info, Borrower>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CollectionsBorrower>, collections: bool) -> Result<()> {
    let borrower_acc = &mut ctx.accounts.borrower_acc;
    borrower_acc.collections = collections;

    emit!(CollectionsBorrowerEvent {
        borrower: borrower_acc.key(),
        collections: collections
    });

    Ok(())
}
