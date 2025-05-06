use anchor_lang::prelude::*;

#[constant]
pub const DICTATOR: Pubkey = pubkey!("Au5UxjuuLLD9AQuE4QWQ1ucUqKPjaXQ8EkSBokUPCiB6");
#[constant]
pub const MINUTE: i64 = 60;
#[constant]
pub const HOUR: i64 = 60 * MINUTE;
#[constant]
pub const DAY: i64 = 24 * HOUR;
#[constant]
pub const USDC_DECIMALS: u8 = 6;
