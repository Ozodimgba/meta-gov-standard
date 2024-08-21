use anchor_lang::prelude::*;
use crate::standard::*;

#[event]
pub struct StateRegistered {
    pub name: String,
    pub government_type: GovernmentType,
    pub program_id: Pubkey,
}


pub fn register_state(ctx: Context<RegisterState>, name: String, gov_type: GovernmentType) -> Result<()> {
    let state_info = &mut ctx.accounts.state_info;
    state_info.name = name.clone();
    state_info.government_type = gov_type;
    state_info.program_id = *ctx.accounts.government_program.key;

    // Generate PDA for the state account
    let seeds = &[b"state", name.as_bytes()];
    let (state_pda, bump) = Pubkey::find_program_address(seeds, ctx.accounts.government_program.key);

    // Prepare CPI to initialize the state in the specific program
    let cpi_program = ctx.accounts.government_program.to_account_info();
    let cpi_accounts = Initialize {
        state: state_pda.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Perform CPI to initialize the state
    standard::initialize(cpi_ctx, name, bump)?;

    // Store the state's PDA in StateInfo
    state_info.state_account = state_pda;

    // Increment state count
    ctx.accounts.meta_state.state_count += 1;

    emit!(StateRegistered {
        name: state_info.name.clone(),
        government_type: state_info.government_type,
        program_id: state_info.program_id,
        state_account: state_info.state_account,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterState<'info> {
    #[account(mut)]
    pub meta_state: Account<'info, MetaState>,
    #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 32)]
    pub state_info: Account<'info, StateInfo>,
    /// CHECK: This is the program ID of the specific government type
    pub government_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct StateInfo {
    pub name: String,
    pub government_type: GovernmentType,
    pub program_id: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Ark {
    pub state_count: u64,
}