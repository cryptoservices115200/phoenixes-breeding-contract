use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Invalid Lava Mint")]
    InvalidLavaMint,

    #[msg("Insufficient Lamports")]
    LackLamports,
}
