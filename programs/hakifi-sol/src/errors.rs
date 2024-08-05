use anchor_lang::error_code;
#[error_code]
pub enum ErrorCode {
    #[msg("Id has been created")]
    InvalidIdHasBeenCreated,
    #[msg("Insufficient balance")]
    InvalidInsufficientBallance,
    #[msg("Mod already exits")]
    ModAlreadyExists,
    #[msg("Mod not exits")]
    ModNotExists,
    #[msg("Not permission")]
    NotPerMission,
}
