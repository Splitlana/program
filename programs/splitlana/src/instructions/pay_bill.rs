use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{errors::SplitError, state::{BillV1, Currency}};

#[derive(Accounts)]
pub struct PayBill<'info> {
    pub payer: Signer<'info>,
    pub bill: Account<'info, BillV1>,
    pub sol_account: Option<SystemAccount<'info>>,
    pub token_account: Option<Account<'info, TokenAccount>>,
    pub system_program: Option<Program<'info, System>>,
    pub token_program: Option<Program<'info, Token>>,
}

impl<'info> PayBill<'info> {
    pub fn pay_bill(&mut self) -> Result<()> {
        let amout_to_pay = self.bill.amount / self.bill.payers.len() as u32;

        //check that payer exists in payers list

        //check the currency of the bill and use appropriate program to transfer funds

        //transfer funds from payer to bill

        //update bill state (paid amount, payer list)
        //Exemplo de USDC
        match self.bill.currency {
            Currency::USDC => {
                require!(self.token_program.is_some(), SplitError::TokenProgramNotProvided);
                let token_progranm = self.token_program.clone().unwrap();
                let token_account = self.token_account.clone().unwrap();
            }
            Currency::SOL => {
                let system_program = self.system_program.clone().unwrap();
                let sol_account = self.sol_account.clone().unwrap();
            }
        }

        Ok(())
    }
}