use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;


declare_id!("5rzxbeLJoSWGw1oDqDBhtDsS3ETgtQpGoNVCNVKUHNmp");

#[program]
pub mod splitlana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
