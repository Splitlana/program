use anchor_lang::prelude::*;

mod instructions;
mod state;
mod errors;

use instructions::*;
use state::*;


declare_id!("AJz2KWMkb3QunBB69w2rDsxpAZD7NzyDDiTaPNugpBnD");
//3hh4gu66UferQzEDHi6enwvVptv8JTQm3U2MHPM2PPpj

#[program]
pub mod splitlana {
    use crate::state::Currency;

    use super::*;

    pub fn init_bill(ctx: Context<InitBill>, _seed: u64, amount: u64, name: String, currency: Currency) -> Result<()> {
        ctx.accounts.init_bill(_seed, amount, name, currency, &ctx.bumps)
    }

    pub fn add_payer(ctx: Context<AddPayer>, payer: Pubkey) -> Result<()> {
        ctx.accounts.add_payer(payer)
    }

    pub fn pay_bill(ctx: Context<PayBill>) -> Result<()> {
        ctx.accounts.pay_bill()
    }

}
