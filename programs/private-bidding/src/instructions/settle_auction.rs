use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

use crate::constants::{BID_ESCROW_SEED, ESCROW_SEED};
use crate::error::ErrorCode;
use crate::{constants::AUCTION_SEED, state::Auction};

#[commit]
#[derive(Accounts)]
pub struct SettleAuction<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [AUCTION_SEED.as_bytes(), auction.asset_mint.key().as_ref(), auction.seller.key().as_ref()],
        bump = auction.bump,
        constraint = !auction.settled @ ErrorCode::AuctionAlreadySettled,
        constraint = auction.bid_count > 0 @ ErrorCode::NoBids,
    )]
    pub auction: Account<'info, Auction>,

    /// CHECK: Winner account (revealed by TEE)
    #[account(mut)]
    pub winner: SystemAccount<'info>,

    /// CHECK: Seller receives payment
    #[account(
        mut,
        constraint = seller.key() == auction.seller @ ErrorCode::InvalidSeller,
    )]
    pub seller: SystemAccount<'info>,

    /// CHECK: Platform fee receiver
    #[account(mut)]
    pub platform: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED.as_bytes(), auction.key().as_ref()],
        bump,
        constraint = escrow_token_account.mint == auction.asset_mint @ ErrorCode::InvalidMint,
        constraint = escrow_token_account.amount >= auction.amount @ ErrorCode::InsufficientEscrow,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = winner_token_account.owner == winner.key() @ ErrorCode::InvalidWinnerTokenAccount,
        constraint = winner_token_account.mint == auction.asset_mint @ ErrorCode::InvalidMint,
    )]
    pub winner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [BID_ESCROW_SEED.as_bytes(), auction.key().as_ref()],
        bump,
    )]
    /// CHECK: PDA escrow for bid funds
    pub auction_escrow: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SettleAuction<'info> {
    pub fn settle_auction(&mut self, bumps: &SettleAuctionBumps) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= self.auction.end_time,
            ErrorCode::AuctionNotEnded
        );

        let winner = self.auction.highest_bidder.ok_or(ErrorCode::NoWinner)?;
        let winning_bid = self.auction.highest_bid_amount;
        let auction_amount = self.auction.amount;
        let auction_bump = self.auction.bump;
        let auction_key = self.auction.key();
        let asset_mint = self.auction.asset_mint;
        let seller_key = self.auction.seller;

        require!(winning_bid > 0, ErrorCode::InvalidBid);
        require!(winner == self.winner.key(), ErrorCode::WinnerMismatch);

        let losing_bids_total = self
            .auction
            .total_bid_pool
            .checked_sub(winning_bid)
            .ok_or(ErrorCode::MathOverflow)?;

        let platform_fee = losing_bids_total
            .checked_mul(20)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(100)
            .ok_or(ErrorCode::MathOverflow)?;

        let winner_bonus = losing_bids_total
            .checked_sub(platform_fee)
            .ok_or(ErrorCode::MathOverflow)?;

        let auction_seeds = &[
            AUCTION_SEED.as_bytes(),
            asset_mint.as_ref(),
            seller_key.as_ref(),
            &[auction_bump],
        ];
        let signer_seeds = &[&auction_seeds[..]];

        let cpi_accounts = Transfer {
            from: self.escrow_token_account.to_account_info(),
            to: self.winner_token_account.to_account_info(),
            authority: self.auction.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token::transfer(cpi_ctx, auction_amount)?;

        let auction_key_ref = auction_key.as_ref();
        let bid_escrow_seeds = &[
            BID_ESCROW_SEED.as_bytes(),
            auction_key_ref,
            &[bumps.auction_escrow],
        ];
        let bid_escrow_signer_seeds = &[&bid_escrow_seeds[..]];

        let cpi_accounts = system_program::Transfer {
            from: self.auction_escrow.to_account_info(),
            to: self.seller.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            bid_escrow_signer_seeds,
        );

        system_program::transfer(cpi_ctx, winning_bid)?;

        let cpi_accounts = system_program::Transfer {
            from: self.auction_escrow.to_account_info(),
            to: self.winner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            bid_escrow_signer_seeds,
        );

        system_program::transfer(cpi_ctx, winner_bonus)?;

        let cpi_accounts = system_program::Transfer {
            from: self.auction_escrow.to_account_info(),
            to: self.platform.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            bid_escrow_signer_seeds,
        );

        system_program::transfer(cpi_ctx, platform_fee)?;

        self.auction.settled = true;

        msg!(
            "Auction settled! Winner: {}, Winning Bid: {}, Winner Bonus: {}, Platform Fee: {}",
            winner,
            winning_bid,
            winner_bonus,
            platform_fee
        );

        commit_and_undelegate_accounts(
            &self.payer,
            vec![&self.auction.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;

        Ok(())
    }
}
