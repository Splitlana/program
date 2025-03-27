use anchor_lang::prelude::*;

use crate::{errors::SplitError, state::{BillV1, Payers}};

// TBD: verify author is the same as the bill author? (if so, require! or constraint?)

#[derive(Accounts)]
pub struct AddPayer<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(mut)]
    pub bill: Account<'info, BillV1>,
}

impl<'info> AddPayer<'info> {
    pub fn add_payer(&mut self, payer: Pubkey) -> Result<()> {

        //check payer is not already listed 
        if self.bill.payers.iter().any(|p| p.payer == payer ) {
            return Err(SplitError::PayerAlreadyExists.into());
        }

        //add new payer to bill
        self.bill.payers.push(Payers {
            payer,
            amount: 0,
            paid: false,
        });

        Ok(())
    }
}

