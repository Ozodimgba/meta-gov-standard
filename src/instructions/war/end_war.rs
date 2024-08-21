// instructions/war/end_war.rs

use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct EndWar<'info> {
    #[account(mut)]
    pub state_a: Account<'info, StateAccount>,
    #[account(mut)]
    pub state_b: Account<'info, StateAccount>,
    #[account(mut)]
    pub war_registry: Account<'info, WarRegistry>,
    pub authority: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum PeaceTerms {
    Surrender,
    Ceasefire,
    TreatyAgreement,
}

   #[event]
   pub struct WarEnded {
        state_a: Pubkey,
        state_b: Pubkey,
        terms: PeaceTerms,
    }

pub fn handle_end_war(ctx: Context<EndWar>, terms: PeaceTerms) -> Result<()> {
    let state_a = &mut ctx.accounts.state_a;
    let state_b = &mut ctx.accounts.state_b;
    let war_registry = &mut ctx.accounts.war_registry;

    // Verify states are at war
    require!(state_a.at_war && state_b.at_war, ErrorCode::NotAtWar);

    // Remove war from registry
    war_registry.wars.retain(|war| 
        !((war.aggressor == *state_a.to_account_info().key && war.defender == *state_b.to_account_info().key) ||
          (war.aggressor == *state_b.to_account_info().key && war.defender == *state_a.to_account_info().key))
    );

    // Update states
    state_a.at_war = false;
    state_b.at_war = false;

    // Apply peace terms
    match terms {
        PeaceTerms::Surrender => {
            // Example: transfer 20% of loser's treasury to winner
            let transfer_amount = state_b.treasury / 5;
            state_b.treasury -= transfer_amount;
            state_a.treasury += transfer_amount;
        },
        PeaceTerms::Ceasefire => {
            // Both states recover some stability
            state_a.stability = (state_a.stability + 10).min(100);
            state_b.stability = (state_b.stability + 10).min(100);
        },
        PeaceTerms::TreatyAgreement => {
            // Example: both states gain some benefit
            state_a.stability = (state_a.stability + 15).min(100);
            state_b.stability = (state_b.stability + 15).min(100);
        },
    }

    // Emit event
    emit!(WarEnded {
        state_a: *state_a.to_account_info().key,
        state_b: *state_b.to_account_info().key,
        terms,
    });

    Ok(())
}