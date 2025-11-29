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
    #[msg("No bids placed on auction")]
    NoBids,
    #[msg("Auction already settled")]
    AuctionAlreadySettled,
    #[msg("Invalid seller account")]
    InvalidSeller,
    #[msg("Invalid mint for escrow account")]
    InvalidMint,
    #[msg("Insufficient funds in escrow account")]
    InsufficientEscrow,
    #[msg("Invalid winner token account")]
    InvalidWinnerTokenAccount,
    #[msg("Invalid auction state")]
    InvalidAuctionState,
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Overflow occurred")]
    Overflow,
    #[msg("Underflow occurred")]
    Underflow,
    #[msg("Invalid bid account")]
    InvalidBidAccount,
     #[msg("Auction has not ended yet")]
    AuctionNotEnded,
    #[msg("No winner found for this auction")]
    NoWinner,
    #[msg("Invalid bid amount")]
    InvalidBid,
    #[msg("Winner account mismatch")]
    WinnerMismatch,
    #[msg("Math overflow occurred")]
    MathOverflow,
    #[msg("Auction has already been settled")]
    AlreadySettled,
    #[msg("Auction is still active")]
    AuctionStillActive,
}
