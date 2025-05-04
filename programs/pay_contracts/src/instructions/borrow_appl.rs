use crate::event::BorrowApplEvent;
use crate::state::{BorrowAppl, Borrower};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(appl_id:String)]
pub struct BorrowApplContx<'info> {
    #[account(mut)]
    pub borrower_signer: Signer<'info>,
    #[account(
        seeds = [
            b"borrower",
            borrower_signer.key().as_ref()
        ],
        bump=borrower_acc.bump
    )]
    pub borrower_acc: Account<'info, Borrower>,
    #[account(
        init,
        payer = borrower_signer,
        space = 8 + BorrowAppl::INIT_SPACE,
        seeds = [
            b"borrower_appl",
            borrower_acc.key().as_ref(),
            appl_id.as_bytes()
        ],
        bump
    )]
    pub borrow_appl: Account<'info, BorrowAppl>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<BorrowApplContx>,
    appl_id: String,
    request_amount: u64,
    metadata: String,
) -> Result<()> {
    let borrow_appl = &mut ctx.accounts.borrow_appl;
    let borrower_acc = &ctx.accounts.borrower_acc;

    borrow_appl.bump = ctx.bumps.borrow_appl;
    borrow_appl.appl_id = appl_id;
    borrow_appl.request_amount = request_amount;
    borrow_appl.approved_amount = 0;
    borrow_appl.metadata = metadata.clone();

    emit!(BorrowApplEvent {
        borrower: borrower_acc.key(),
        request_amount: request_amount,
        metadata: metadata
    });

    Ok(())
}
