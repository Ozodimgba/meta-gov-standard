// src/state/mod.rs

use anchor_lang::prelude::*;

#[account]
pub struct StateAccount {
    pub name: String,
    pub government_type: GovernmentType,
    pub stability: u8,
    pub treasury: u64,
    pub at_war: bool,
    // Add other fields as necessary
}

#[account]
pub struct WarRegistry {
    pub wars: Vec<War>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct War {
    pub aggressor: Pubkey,
    pub defender: Pubkey,
    pub start_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum GovernmentType {
    Monarchy,
    Democracy,
    // Add other government types
}

// Add other state-related structs and enums here