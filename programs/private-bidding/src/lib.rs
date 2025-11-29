use anchor_lang::prelude::*;

mod constants;
mod error;
mod instructions;
mod state;

use constants::*;
use instructions::*;
use state::*;

declare_id!("CQ17v9aregimgfUchRgfx2uSEsPcekGdKvu9NaU5C1Xs");

#[program]
pub mod private_bidding {
    use super::*;

    pub fn create_auction(
        ctx: Context<CreateAuction>,
        asset_type: AssetType,
        end_time: i64,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .create_auction(asset_type, end_time, amount, &ctx.bumps)?;
        ctx.accounts.deposit_asset()?;
        Ok(())
    }
}
