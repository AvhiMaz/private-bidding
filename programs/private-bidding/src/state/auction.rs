use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Auction {
    pub seller: Pubkey,
    pub asset_mint: Pubkey,
    pub asset_type: AssetType,

    pub start_time: i64,
    pub end_time: i64,

    pub min_bid_amount: u64,
    pub highest_bid_amount: u64,
    pub bid_count: u32,

    pub highest_bidder: Option<Pubkey>,
    pub settled: bool,
    #[max_len(200)]
    pub tee_winner_proof: Option<Vec<u8>>,

    pub bump: u8,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq, InitSpace)]
pub enum AssetType {
    Nft,
    Token,
    CompressedNft,
    SolDomain,
}
