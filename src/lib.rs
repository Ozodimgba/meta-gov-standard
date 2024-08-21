use anchor_lang::prelude::*;
mod standard;
mod instructions;
mod state;
mod error;


use standard::*;

use instructions::register::*;
use instructions::war::*;
use state::*;
pub use error::ErrorCode;

declare_id!("D8a4MAvGM74DuqTVczDVpFLsBtsqQuF3U3GXmzeYdt4z");

#[program]
pub mod ark {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.ark.state_count = 0;
        Ok(())
    }

    pub fn register_state(ctx: Context<RegisterState>, name: String, gov_type: standard::GovernmentType) -> Result<()> {
        instructions::register::register_state(ctx, name, gov_type)
    }

    ///add war ixs here
    pub fn declare_war(ctx: Context<DeclareWar>, reason: String) -> Result<()> {
        instructions::war::declare_war::handle_declare_war(ctx, reason)
    }

    pub fn perform_war_action(ctx: Context<PerformWarAction>, action: WarAction) -> Result<()> {
        instructions::war::perform_war_action::handle_perform_war_action(ctx, action)
    }

    pub fn end_war(ctx: Context<EndWar>, terms: PeaceTerms) -> Result<()> {
        instructions::war::end_war::handle_end_war(ctx, terms)
    }

    pub fn propose_alliance(ctx: Context<ProposeAlliance>) -> Result<()> {
        require!(ctx.accounts.proposer.key() != ctx.accounts.proposed.key(), ErrorCode::CannotAllySelf);

        emit!(AllianceProposed {
            proposer: ctx.accounts.proposer.key(),
            proposed: ctx.accounts.proposed.key(),
        });

        Ok(())
    }

    // pub fn broadcast_global_event(ctx: Context<BroadcastGlobalEvent>, event: GlobalEvent) -> Result<()> {
    //     // In a real implementation, you would iterate through all registered states
    //     // and call apply_global_event on each one. For brevity, we'll just emit an event here.
    //     emit!(GlobalEventBroadcast { event });
    //     Ok(())
    // }
}

// ... (account structs, errors, and events definitions)

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 8)]
    pub ark: Account<'info, Ark>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ProposeAlliance<'info> {
    pub proposer: Account<'info, StateInfo>,
    pub proposed: Account<'info, StateInfo>,
    pub authority: Signer<'info>,
}

#[account]
pub struct RegistryEntry {
    pub name: String,
    pub government_type: GovernmentType,
    pub program_id: Pubkey,
    pub state_account: Pubkey,
}

#[account]
pub struct Ark {
    pub state_count: u64,
}

#[account]
pub struct StateInfo {
    pub name: String,
    pub government_type: GovernmentType,
    pub program_id: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum GovernmentType {
    Monarchy,
    Democracy,
    // Add more as needed
}


#[event]
pub struct AllianceProposed {
    pub proposer: Pubkey,
    pub proposed: Pubkey,
}