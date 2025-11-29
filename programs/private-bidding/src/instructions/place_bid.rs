use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::system_program::Transfer;

use crate::{
    constants::{ANCHOR_DISCRIMINATOR_SIZE, AUCTION_SEED, BID_ESCROW_SEED, BID_SEED},
    state::{Auction, Bid},
};

use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        seeds = [AUCTION_SEED.as_bytes(), auction.asset_mint.key().as_ref(), auction.seller.key().as_ref()],
        bump = auction.bump,
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        init,
        payer = bidder,
        space = ANCHOR_DISCRIMINATOR_SIZE + Bid::INIT_SPACE,
        seeds = [BID_SEED.as_bytes(), auction.key().as_ref(), bidder.key().as_ref()],
        bump,
    )]
    pub bid: Account<'info, Bid>,

    #[account(
        mut,
        seeds = [BID_ESCROW_SEED.as_bytes(), auction.key().as_ref()],
        bump,

    )]
    /// CHECK: Shared escrow for all auction bids
    pub auction_escrow: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBid<'info> {
    pub fn place_bid(&mut self, amount: u64, bumps: &PlaceBidBumps) -> Result<()> {
        require!(
            Clock::get()?.unix_timestamp < self.auction.end_time,
            ErrorCode::AuctionEnded
        );

        require!(!self.auction.settled, ErrorCode::AuctionSettled);

        require!(amount >= self.auction.min_bid_amount, ErrorCode::BidTooLow);

        require!(
            self.bidder.key() != self.auction.seller,
            ErrorCode::SellerCannotBid
        );

        self.bid.set_inner(Bid {
            auction: self.auction.key(),
            bidder: self.bidder.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
            bump: bumps.bid,
        });

        let cpi_account = Transfer {
            from: self.bidder.to_account_info(),
            to: self.auction_escrow.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_account);

        system_program::transfer(cpi_ctx, amount)?;

        self.auction.bid_count += 1;
        self.auction.total_bid_pool += amount;

        Ok(())
    }
}
