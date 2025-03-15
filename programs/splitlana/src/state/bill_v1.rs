use anchor_lang::prelude::*;

#[account]
pub struct BillV1 {
    pub author: Pubkey,
    pub name: String,
    pub payers: Vec<Payers>,
    pub amount: u32,
    pub paid: u32,
    pub currency: Currency,
}

impl Space for BillV1 {
    const INIT_SPACE: usize = 8 + 32 + (4 + 10) + 4 + 4;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, PartialEq, Debug)]
pub struct Payers {
    pub payer: Pubkey,
    pub amount: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum Currency {
    SOL = 0,
    USDC = 1,
}