use anchor_lang::prelude::*;

// This trait defines the standard interface that all government programs must implement
pub trait GovernanceStandard {
    fn initialize(ctx: Context<Initialize>, name: String) -> Result<()>;
    fn get_state_info(ctx: Context<GetStateInfo>) -> Result<StateInfo>;
    fn apply_global_event(ctx: Context<ApplyGlobalEvent>, event: GlobalEvent) -> Result<()>;
    fn receive_declaration_of_war(ctx: Context<ReceiveWarDeclaration>, from_state: Pubkey) -> Result<()>;
    fn receive_alliance_proposal(ctx: Context<ReceiveAllianceProposal>, from_state: Pubkey) -> Result<()>;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct StateInfo {
    pub name: String,
    pub government_type: GovernmentType,
    pub stability: u8,
    pub treasury: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum GovernmentType {
    Monarchy,
    Democracy,
    // Add more as needed
}

impl Default for GovernmentType {
    fn default() -> Self {
        GovernmentType::Monarchy 
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum GlobalEvent {
    NaturalDisaster,
    EconomicBoom,
    Pandemic,
    // Add more as needed
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetStateInfo<'info> {
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApplyGlobalEvent<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ReceiveWarDeclaration<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ReceiveAllianceProposal<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
}