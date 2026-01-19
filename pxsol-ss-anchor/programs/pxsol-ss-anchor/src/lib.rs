use anchor_lang::prelude::*;
use anchor_lang::system_program;


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
    
    #[derive(Accounts)]
    pub struct Init<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(
            init,
            payer = user,
            seeds = [b"seeds", user.key().as_ref()],
            bump,
            space = Data::space_for(0)
        )]
        pub user_pda: Account<'info, Data>,
        pub system_program: Program<'info, System>,

    }
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

    pub fn update(ctx: Context<Update>, data: Vec<u8>) -> Result<()> {
        let account_user = &ctx.accounts.user;
        let account_user_pda = &mut ctx.accounts.user_pda;
        // Authorization: only the stored authority can update.
        require_keys_eq!(account_user_pda.auth, account_user.key(), PxsolError::Unauthorized);
        // At this point, Anchor has already reallocated the account according to the `realloc = ...` constraint
        // (using `new_data.len()`), pulling extra lamports from auth if needed to maintain rent-exemption.
        account_user_pda.data = data;
        // If the account was shrunk, Anchor won't automatically refund excess lamports. Refund any surplus (over the
        // new rent-exempt minimum) back to the user.
        let account_user_pda_info = account_user_pda.to_account_info();
        let rent = Rent::get()?;
        let rent_exemption = rent.minimum_balance(account_user_pda_info.data_len());
        let hold = **account_user_pda_info.lamports.borrow();
        if hold > rent_exemption {
            let refund = hold.saturating_sub(rent_exemption);
            // Transfer lamports from PDA to user using the PDA as signer.
            let signer_seeds: &[&[u8]] = &[b"seeds", account_user.key.as_ref(), &[account_user_pda.bump]];
            let signer = &[signer_seeds];
            let cpictx = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer { from: account_user_pda_info.clone(), to: account_user.to_account_info() },
                signer,
            );
            // It's okay if refund equals current - min_rent; system program enforces balances.
            system_program::transfer(cpictx, refund)?;
        }
        Ok(())
    }
    

#[derive(Accounts)]
#[instruction(new_data: Vec<u8>)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"seeds", user.key().as_ref()],
        bump = user_pda.bump,
        realloc = Data::space_for(new_data.len()),
        realloc::payer = user,
        realloc::zero = false,
        constraint = user_pda.auth == user.key() @ PxsolError::Unauthorized,
    )]
    pub user_pda: Account<'info, Data>,
    pub system_program: Program<'info, System>,
}
#[error_code]
pub enum PxsolError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    
    // 可以添加更多错误
    #[msg("Invalid data provided")]
    InvalidData,
}
}




