use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[constant]
pub const AUCTION_SEED: &str = "auction";
pub const ESCROW_SEED: &str = "escrow";
pub const BID_SEED: &str = "bid";
pub const BID_ESCROW_SEED: &str = "bid_escrow";
