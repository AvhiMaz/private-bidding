use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::ephemeral;

mod constants;
mod error;
mod instructions;
mod state;

use constants::*;
use instructions::*;
use state::*;

declare_id!("CQ17v9aregimgfUchRgfx2uSEsPcekGdKvu9NaU5C1Xs");

#[ephemeral]
#[program]
pub mod private_bidding {
    use ephemeral_rollups_sdk::{
        access_control::{CreateGroupCpiBuilder, CreatePermissionCpiBuilder},
        cpi::DelegateConfig,
    };

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

    pub fn place_bid(ctx: Context<PlaceBid>, amount: u64) -> Result<()> {
        ctx.accounts.place_bid(amount, &ctx.bumps)?;
        Ok(())
    }

    pub fn delegate_bid(ctx: Context<DelegateBid>) -> Result<()> {
        ctx.accounts.delegate_bid_pda(
            &ctx.accounts.bidder,
            &[
                BID_SEED.as_bytes(),
                ctx.accounts.auction.key().as_ref(),
                ctx.accounts.bidder.key().as_ref(),
            ],
            DelegateConfig {
                validator: Some(TEE_VALIDATOR),
                ..Default::default()
            },
        )?;
        Ok(())
    }

    pub fn create_bid_permission(
        ctx: Context<CreateBidPermission>,
        group_id: Pubkey,
    ) -> Result<()> {
        CreateGroupCpiBuilder::new(&ctx.accounts.permission_program)
            .group(&ctx.accounts.group)
            .id(group_id)
            .members(vec![ctx.accounts.bidder.key()])
            .payer(&ctx.accounts.bidder)
            .system_program(&ctx.accounts.system_program)
            .invoke()?;

        CreatePermissionCpiBuilder::new(&ctx.accounts.permission_program)
            .permission(&ctx.accounts.permission)
            .delegated_account(&ctx.accounts.bid.to_account_info())
            .group(&ctx.accounts.group)
            .payer(&ctx.accounts.bidder)
            .system_program(&ctx.accounts.system_program)
            .invoke_signed(&[&[
                BID_SEED.as_bytes(),
                ctx.accounts.auction.key().as_ref(),
                ctx.accounts.bidder.key().as_ref(),
                &[ctx.bumps.bid],
            ]])?;

        Ok(())
    }

    pub fn settle_auction(ctx: Context<SettleAuction>) -> Result<()> {
        ctx.accounts.settle_auction(&ctx.bumps)?;
        Ok(())
    }
}
