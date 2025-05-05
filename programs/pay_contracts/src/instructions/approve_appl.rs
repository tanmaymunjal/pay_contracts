use crate::error::PayContractsError;
use crate::event::ApproveApl;
use crate::state::{BorrowAppl, InitializeAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApproveBorrowApl<'info> {
    #[account(constraint = lending_agent.key() == initialize_acc.lending_agent)]
    pub lending_agent: Signer<'info>,
    #[account(mut)]
    pub borrow_appl: Account<'info, BorrowAppl>,
    #[account(
        seeds = [
            b"initialize"
        ],
        bump=initialize_acc.bump
    )]
    pub initialize_acc: Account<'info, InitializeAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ApproveBorrowApl>, approved_amount: u64) -> Result<()> {
    let borrow_appl = &mut ctx.accounts.borrow_appl;

    borrow_appl.approved_amount += approved_amount;
    require!(
        borrow_appl.approved_amount <= borrow_appl.request_amount,
        PayContractsError::ApprovalAmountTooHigh
    );

    emit!(ApproveApl {
        borrow_appl: borrow_appl.key(),
        approval_amount: approved_amount
    });

    Ok(())
}
