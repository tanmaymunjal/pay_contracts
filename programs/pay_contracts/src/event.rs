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
    pub debarred: bool,
}

#[event]
pub struct CollectionsBorrowerEvent {
    pub borrower: Pubkey,
    pub collections: bool,
}

#[event]
pub struct BorrowApplEvent {
    pub borrower: Pubkey,
    pub request_amount: u64,
    pub metadata: String,
}
