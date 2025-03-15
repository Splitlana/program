use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitBill<'info> {
    pub user: Signer<'info>,
    // todo!("Panadas implement this");
}