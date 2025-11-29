use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[constant]
pub const AUCTION_SEED: &str = "auction";
pub const ESCROW_SEED: &str = "escrow";