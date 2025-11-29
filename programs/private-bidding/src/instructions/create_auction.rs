use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{state::Auction, AssetType, ANCHOR_DISCRIMINATOR_SIZE, AUCTION_SEED, ESCROW_SEED};

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        payer = seller,
        space = ANCHOR_DISCRIMINATOR_SIZE + Auction::INIT_SPACE,
        seeds = [AUCTION_SEED.as_bytes(), asset_mint.key().as_ref(), seller.key().as_ref()],
        bump,
    )]
    pub auction: Account<'info, Auction>,

    /// CHECK: Asset mint being auctioned
    pub asset_mint: AccountInfo<'info>,

    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
        constraint = seller_token_account.mint == asset_mint.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = seller,
        seeds = [ESCROW_SEED.as_bytes(), auction.key().as_ref()],
        bump,
        token::mint = asset_mint,
        token::authority = auction,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateAuction<'info> {
    pub fn create_auction(
        &mut self,
        asset_type: AssetType,
        end_time: i64,
        amount: u64,
        bumps: &CreateAuctionBumps,
    ) -> Result<()> {
        self.auction.set_inner(Auction {
            seller: self.seller.key(),
            asset_mint: self.asset_mint.key(),
            asset_type,
            amount,
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

    pub fn deposit_asset(&mut self) -> Result<()> {
        let cpi_account = Transfer {
            from: self.seller_token_account.to_account_info(),
            to: self.escrow_token_account.to_account_info(),
            authority: self.seller.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_account);

        token::transfer(cpi_ctx, self.auction.amount)?;

        Ok(())
    }
}
