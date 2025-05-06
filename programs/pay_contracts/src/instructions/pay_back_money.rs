use crate::constants::USDC_DECIMALS;
use crate::error::PayContractsError;
use crate::event::MoneyPaidBack;
use crate::state::{BorrowAppl, Borrower, InitializeAccount, TreasuryAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct PayBackMoney<'info> {
    #[account(constraint = fraud_agent.key() == initialize_acc.fraud_agent)]
    pub fraud_agent: Signer<'info>,
    #[account(mut)]
    pub borrower_signer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = borrower_signer,
        associated_token::token_program = token_program,
    )]
    pub borrower_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            b"borrower",
            borrower_signer.key().as_ref()
        ],
        bump=borrower_acc.bump
    )]
    pub borrower_acc: Account<'info, Borrower>,
    #[account(
        mut,
        seeds = [
            b"borrower_appl",
            borrower_acc.key().as_ref(),
            borrow_appl.appl_id.as_bytes()
        ],
        bump=borrow_appl.bump
    )]
    pub borrow_appl: Account<'info, BorrowAppl>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury_acc.bump
    )]
    pub treasury_acc: Account<'info, TreasuryAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury_acc,
        associated_token::token_program = token_program,
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"initialize"],
        bump = initialize_acc.bump
    )]
    pub initialize_acc: Account<'info, InitializeAccount>,

    // #[account(address = USDC)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PayBackMoney>, returned: u64, interest: u64) -> Result<()> {
    let treasury_acc = &mut ctx.accounts.treasury_acc;
    let treasury_token_account = &mut ctx.accounts.treasury_token_account;
    let borrow_appl = &mut ctx.accounts.borrow_appl;
    let borrower_signer = &mut ctx.accounts.borrower_signer;
    let usdc_mint = &ctx.accounts.usdc_mint;
    let borrower_token_account = &mut ctx.accounts.borrower_token_account;

    treasury_acc.available_funds += returned + interest;
    require!(
        borrow_appl.used_amount >= returned,
        PayContractsError::CanNotReturnMoreThanUsed
    );
    borrow_appl.used_amount -= returned;

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: borrower_token_account.to_account_info(),
                to: treasury_token_account.to_account_info(),
                authority: borrower_signer.to_account_info(),
                mint: usdc_mint.to_account_info(),
            },
        ),
        returned + interest,
        USDC_DECIMALS,
    )?;

    emit!(MoneyPaidBack {
        borrow_appl: borrow_appl.key(),
        returned,
        interest
    });

    Ok(())
}
