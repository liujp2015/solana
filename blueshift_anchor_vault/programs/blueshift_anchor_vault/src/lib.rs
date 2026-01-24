use anchor_lang::{prelude::*, solana_program};
use solana_program::{
    program::{invoke_signed},
    system_instruction,
};

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // Ensure amount is valid
        require_gt!(amount, 0, VaultError::InvalidAmount);

        // Transfer SOL from signer to vault PDA
        let transfer_ix = system_instruction::transfer(
            ctx.accounts.signer.key,
            ctx.accounts.vault.key,
            amount,
        );

        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        let vault_lamports = ctx.accounts.vault.lamports();
        require_gt!(vault_lamports, 0, VaultError::InvalidAmount);
    
        // Build the seed slice correctly - note the different structure
        let signer_seeds: &[&[&[u8]]] = &[
            &[
                b"vault",
                ctx.accounts.signer.key.as_ref(),
                &[ctx.bumps.vault],
            ],
        ];
    
        let transfer_ix = system_instruction::transfer(
            ctx.accounts.vault.key,
            ctx.accounts.signer.key,
            vault_lamports,
        );
    
        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer_seeds,
        )?;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>, // Better to use SystemAccount instead of UncheckedAccount
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Invalid amount")]
    InvalidAmount,
}