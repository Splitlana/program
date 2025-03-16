use anchor_lang::prelude::*;

use crate::state::{BillV1, Currency};

#[derive(Accounts)]
#[instruction(_seed: u64)]
pub struct InitBill<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"bill", user.key().as_ref(), &_seed.to_le_bytes()],
        bump,
        space = BillV1::INIT_SPACE
    )]
    pub bill: Account<'info, BillV1>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitBill<'info> {
    pub fn init_bill(&mut self, _seed: u64, amount: u64, name: String, currency: Currency, bumps: &InitBillBumps) -> Result<()> {
        self.bill.set_inner({
            BillV1 {
                author: self.user.key(),
                name,
                payers: vec![],
                paid: 0,
                amount,
                currency,
                bump: bumps.bill,
            }
        });
        
        Ok(())
    }
}