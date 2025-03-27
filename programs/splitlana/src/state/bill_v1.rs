use anchor_lang::prelude::*;

#[account]
pub struct BillV1 {
    pub author: Pubkey,
    pub name: String,
    pub payers: Vec<Payers>,
    pub total_amount: u64,
    pub total_paid: u64,
    pub currency: Currency,
    pub bump: u8,
}
impl Space for BillV1 {
    const INIT_SPACE: usize = 8 + 32 + ((4 + (32 + 4 + 1)) * 10) + (4 + 10) + 4 + 4 + 1 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, PartialEq, Debug)]
pub struct Payers {
    pub payer: Pubkey,
    pub amount: u64,
    pub paid: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum Currency {
    SOL = 0,
    USDC = 1,
}