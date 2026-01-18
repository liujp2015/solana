use anchor_lang::prelude::*;

declare_id!("2KzMeNKbe8pKqT9aMhQpQuNHH9JkMFwNNANxQHvUoCko");

#[program]
pub mod pxsol_ss_anchor {
    use super::*;
    pub fn init(ctx: Context<Init>) -> Result<()> {
        let account_user = &ctx.accounts.user;
        let account_user_pda = &mut ctx.accounts.user_pda;
        account_user_pda.auth = account_user.key();
        account_user_pda.bump = ctx.bumps.user_pda;
        account_user_pda.data = Vec::new();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init {}



#[account]
pub struct Data {
    pub auth: Pubkey, // The owner of this pda account
    pub bump: u8,     // The bump to generate the PDA
    pub data: Vec<u8> // The content, arbitrary bytes
}

impl Data {
    pub fn space_for(data_len: usize) -> usize {
        // 8 (discriminator) + 32 (auth) + 1 (bump) + 4 (vec len) + data_len
        8 + 32 + 1 + 4 + data_len
    }
}