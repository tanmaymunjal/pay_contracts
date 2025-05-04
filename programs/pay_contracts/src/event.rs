use anchor_lang::prelude::*;

#[event]
pub struct ContractInitialized {
    pub card_provider: Pubkey,
    pub lending_agent: Pubkey,
    pub fraud_agent: Pubkey,
}

#[event]
pub struct ContractEdited {
    pub card_provider: Pubkey,
    pub lending_agent: Pubkey,
    pub fraud_agent: Pubkey,
}

#[event]
pub struct BorrowerCreated {
    pub borrower: Pubkey,
}

#[event]
pub struct BorrowerDeBarred {
    pub borrower: Pubkey,
    pub debarred: bool
}