// instructions/war/declare_war.rs

use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct DeclareWar<'info> {
    #[account(mut)]
    pub aggressor: Account<'info, StateAccount>,
    pub defender: Account<'info, StateAccount>,
    #[account(mut)]
    pub war_registry: Account<'info, WarRegistry>,
    pub authority: Signer<'info>,
}

#[event]
pub struct WarDeclared {
    pub aggressor: Pubkey,
    pub defender: Pubkey,
    pub reason: String,
}

pub fn handle_declare_war(ctx: Context<DeclareWar>, reason: String) -> Result<()> {
    let aggressor = &mut ctx.accounts.aggressor;
    let defender = &ctx.accounts.defender;
    let war_registry = &mut ctx.accounts.war_registry;

    // Verify the aggressor is not already at war
    require!(!aggressor.at_war, ErrorCode::AlreadyAtWar);

    // Update WarRegistry
    war_registry.wars.push(War {
        aggressor: *aggressor.to_account_info().key,
        defender: *defender.to_account_info().key,
        start_time: Clock::get()?.unix_timestamp,
    });

    // Update aggressor state
    aggressor.at_war = true;
    aggressor.stability = aggressor.stability.saturating_sub(10);

    // Emit event
    emit!(WarDeclared {
        aggressor: *aggressor.to_account_info().key,
        defender: *defender.to_account_info().key,
        reason,
    });

    Ok(())
}