use anchor_lang::error_code;
use solana_program::msg;

#[error_code]
pub enum SplitError {
    #[msg("Token Program Not Provided")]
    TokenProgramNotProvided,

    #[msg("Payer Not In List")]
    PayerNotInList
}