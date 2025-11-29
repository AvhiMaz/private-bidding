use anchor_lang::prelude::*;

use crate::{state::Auction, AssetType, ANCHOR_DISCRIMINATOR_SIZE, SEED};

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        payer = seller,
        space = ANCHOR_DISCRIMINATOR_SIZE + Auction::INIT_SPACE,
        seeds = [SEED.as_bytes(), asset_mint.key().as_ref(), seller.key().as_ref()],
        bump,
    )]
    pub auction: Account<'info, Auction>,

    /// CHECK: Asset mint being auctioned
    pub asset_mint: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateAuction<'info> {
    pub fn create_auction(
        &mut self,
        asset_type: AssetType,
        end_time: i64,
        bumps: &CreateAuctionBumps,
    ) -> Result<()> {
        self.auction.set_inner(Auction {
            seller: self.seller.key(),
            asset_mint: self.asset_mint.key(),
            asset_type,
            start_time: Clock::get()?.unix_timestamp,
            end_time,
            min_bid_amount: 0,
            highest_bid_amount: 0,
            bid_count: 0,
            highest_bidder: None,
            settled: false,
            tee_winner_proof: None,
            bump: bumps.auction,
        });
        Ok(())
    }
}
