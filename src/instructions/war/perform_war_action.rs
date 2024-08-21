// instructions/war/perform_war_action.rs

use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct PerformWarAction<'info> {
    #[account(mut)]
    pub attacker: Account<'info, StateAccount>,
    #[account(mut)]
    pub defender: Account<'info, StateAccount>,
    #[account(mut)]
    pub war_registry: Account<'info, WarRegistry>,
    pub authority: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum WarAction {
    MilitaryStrike,
    EconomicSanction,
    Espionage,
}

#[event]
pub struct WarActionPerformed {
    attacker: Pubkey,
    defender: Pubkey,
    action: WarAction,
}

pub fn handle_perform_war_action(ctx: Context<PerformWarAction>, action: WarAction) -> Result<()> {
    let attacker = &mut ctx.accounts.attacker;
    let defender = &mut ctx.accounts.defender;
    let war_registry = &ctx.accounts.war_registry;

    // Verify states are at war
    require!(attacker.at_war && defender.at_war, ErrorCode::NotAtWar);
    require!(
        war_registry.wars.iter().any(|war| 
            (war.aggressor == *attacker.to_account_info().key && war.defender == *defender.to_account_info().key) ||
            (war.aggressor == *defender.to_account_info().key && war.defender == *attacker.to_account_info().key)
        ),
        ErrorCode::WarNotRegistered
    );

    // Apply action effects
    match action {
        WarAction::MilitaryStrike => {
            defender.stability = defender.stability.saturating_sub(15);
            attacker.stability = attacker.stability.saturating_sub(5);
        },
        WarAction::EconomicSanction => {
            defender.treasury = defender.treasury.saturating_sub(defender.treasury / 10);
            attacker.treasury = attacker.treasury.saturating_sub(attacker.treasury / 20);
        },
        WarAction::Espionage => {
            defender.stability = defender.stability.saturating_sub(5);
            // Espionage effect could be more complex, this is a simple example
        },
    }

    // Emit event
    emit!(WarActionPerformed {
        attacker: *attacker.to_account_info().key,
        defender: *defender.to_account_info().key,
        action,
    });

    Ok(())
}