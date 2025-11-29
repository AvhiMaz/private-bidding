use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
pub const TEE_VALIDATOR: Pubkey = pubkey!("FnE6VJT5QNZdedZPnCoLsARgBwoE6DeJNjBs2H1gySXA");

#[constant]
pub const AUCTION_SEED: &str = "auction";
pub const ESCROW_SEED: &str = "escrow";
pub const BID_SEED: &str = "bid";
pub const BID_ESCROW_SEED: &str = "bid_escrow";
