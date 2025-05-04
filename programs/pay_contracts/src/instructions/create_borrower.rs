use crate::event::BorrowerCreated;
use crate::state::Borrower;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateBorrower<'info> {
    #[account(mut)]
    pub borrower_signer: Signer<'info>,
    #[account(
        init,
        payer=borrower_signer,
        space=8+Borrower::INIT_SPACE,
        seeds = [
            b"borrower",
            borrower_signer.key().as_ref()
        ],
        bump
    )]
    pub borrower_acc: Account<'info, Borrower>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateBorrower>) -> Result<()> {
    let borrower_acc = &mut ctx.accounts.borrower_acc;
    let borrower_signer = &ctx.accounts.borrower_signer;

    borrower_acc.bump = ctx.bumps.borrower_acc;
    borrower_acc.total_borrowed_amount = 0;
    borrower_acc.debarred = false;
    borrower_acc.collections = false;

    emit!(BorrowerCreated {
        borrower: borrower_signer.key()
    });

    Ok(())
}
