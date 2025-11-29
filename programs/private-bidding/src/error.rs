use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Auction has ended")]
    AuctionEnded,
    #[msg("Auction already settled")]
    AuctionSettled,
    #[msg("Bid amount too low")]
    BidTooLow,
    #[msg("Seller cannot bid on own auction")]
    SellerCannotBid,
}
