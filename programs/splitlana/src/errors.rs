use anchor_lang::error_code;
use solana_program::msg;

#[error_code]
pub enum SplitError {
    #[msg("Accounts provided are not valid")]
    InvalidAccounts,

    #[msg("Payer Not In List")]
    PayerNotInList,

    #[msg("Payer list is empty")]
    PayerListEmpty,

    #[msg("Payer already paid")]
    PayerAlreadyPaid,

    #[msg("Payer already exists")]
    PayerAlreadyExists,
    
}