use anchor_lang::prelude::*;
use anchor_spl::token::{
        Token, 
        TokenAccount, 
        Transfer as SplTransfer, 
        transfer as spl_transfer
    };
use anchor_lang::system_program::{Transfer, transfer};

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
        //calculate amount to pay: even split among payers of total amount
        require!(self.bill.payers.len() > 0, SplitError::PayerListEmpty);
        let amount_to_pay = self.bill.total_amount / self.bill.payers.len() as u64;

        //check that payer exists in payers list
        let mut payers = self.bill.payers.clone();
        let payer = payers.iter_mut().find(|payer| payer.payer == *self.payer.key);
        if payer.is_none() {
            return Err(SplitError::PayerNotInList.into());
        }
        //check that payer has not already paid
        if payer.as_ref().unwrap().paid {
            return Err(SplitError::PayerAlreadyPaid.into());
        }

        //check the currency of the bill and use appropriate program to transfer funds
        match self.bill.currency {
            Currency::USDC => {
                if let (Some(token_program), Some(token_account)) = (self.token_program.as_ref(), self.payer_token_account.as_ref()) {
                    let cpi_accounts = SplTransfer {
                        from: token_account.to_account_info(),
                        to: self.author_token_account.as_ref().unwrap().to_account_info(),
                        authority: self.payer.to_account_info(),
                    };

                    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);

                    // transfer funds from payer token account to author token account
                    spl_transfer(cpi_ctx, amount_to_pay)?;

                } else {
                    return Err(SplitError::InvalidAccounts.into());
                }
            }
            Currency::SOL => {
                if let (Some(system_program), Some(author)) = (self.system_program.as_ref(), self.author.as_ref()) {
                    let cpi_accounts = Transfer {
                        from: self.sol_account.as_ref().unwrap().to_account_info(),
                        to: author.to_account_info(),
                    };

                    let cpi_ctx = CpiContext::new(system_program.to_account_info(), cpi_accounts);

                    // transfer funds from payer to author
                    transfer(cpi_ctx, amount_to_pay)?;
                } else {
                    return Err(SplitError::InvalidAccounts.into());
                }
            }
        };

        //update bill state (paid amount, payer list)
        self.bill.total_paid += amount_to_pay;
        if let Some(payer) = payer {
            payer.paid = true;
            payer.amount += amount_to_pay;
        }
        

        Ok(())
    }
}