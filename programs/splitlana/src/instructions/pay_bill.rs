use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::{errors::SplitError, state::{BillV1, Currency}};

#[derive(Accounts)]
pub struct PayBill<'info> {
    pub payer: Signer<'info>,
    pub author: Option<SystemAccount<'info>>,
    pub bill: Account<'info, BillV1>,
    pub sol_account: Option<SystemAccount<'info>>,
    pub payer_token_account: Option<Account<'info, TokenAccount>>,
    pub author_token_account: Option<Account<'info, TokenAccount>>,
    pub system_program: Option<Program<'info, System>>,
    pub token_program: Option<Program<'info, Token>>,
}

impl<'info> PayBill<'info> {
    pub fn pay_bill(&mut self) -> Result<()> {
        let amount_to_pay = self.bill.amount / self.bill.payers.len() as u64;

        //check that payer exists in payers list
        require!(self.bill.payers.iter().find(|payer| payer.payer == *self.payer.key).is_some(), SplitError::PayerNotInList);

        //check the currency of the bill and use appropriate program to transfer funds
        match self.bill.currency {
            Currency::USDC => {
                if let (Some(token_program), Some(token_account)) = (self.token_program.clone(), self.payer_token_account.clone()) {
                    let cpi_accounts = Transfer {
                        from: token_account.to_account_info(),
                        to: self.author_token_account.unwrap().to_account_info(),
                        authority: self.payer.to_account_info(),
                    };

                    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);

                    transfer(cpi_ctx, amount_to_pay)?;
                } else {
                    return Err(SplitError::TokenProgramNotProvided.into());
                }
            }
            Currency::SOL => {
                let system_program = self.system_program.clone().unwrap();
                let sol_account = self.sol_account.clone().unwrap();
            }
        }


        //transfer funds from payer to bill

        //update bill state (paid amount, payer list)

        Ok(())
    }
}