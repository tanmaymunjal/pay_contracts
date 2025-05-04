use anchor_lang::prelude::*;

#[event]
pub struct ContractInitialized {
    pub card_provider: Pubkey, 
    pub lending_agent: Pubkey, 
    pub fraud_agent: Pubkey
}
