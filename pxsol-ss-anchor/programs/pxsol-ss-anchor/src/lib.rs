use anchor_lang::prelude::*;

declare_id!("2KzMeNKbe8pKqT9aMhQpQuNHH9JkMFwNNANxQHvUoCko");

#[program]
pub mod pxsol_ss_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
