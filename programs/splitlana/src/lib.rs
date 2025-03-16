use anchor_lang::prelude::*;

mod instructions;
mod state;
mod errors;

use instructions::*;
use state::*;


declare_id!("5rzxbeLJoSWGw1oDqDBhtDsS3ETgtQpGoNVCNVKUHNmp");

#[program]
pub mod splitlana {
    use crate::state::Currency;

    use super::*;

    pub fn init_bill(ctx: Context<InitBill>, _seed: u64, amount: u64, name: String, currency: Currency) -> Result<()> {
        ctx.accounts.init_bill(_seed, amount, name, currency, &ctx.bumps)
    }
}
