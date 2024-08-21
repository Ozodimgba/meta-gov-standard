use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("A state cannot declare war on itself")]
    CannotWarSelf,
    #[msg("A state cannot form an alliance with itself")]
    CannotAllySelf,
    #[msg("This state is already at war")]
    AlreadyAtWar,
    #[msg("This state is not at war")]
    NotAtWar,
    #[msg("This war is not registered in the war registry")]
    WarNotRegistered,
    #[msg("Unauthorized action")]
    Unauthorized,
    // Add any other error codes you need
}