#![allow(unexpected_cfgs)]
solana_program::entrypoint!(process_instruction);
pub fn process_instruction_mint(
    program_id: &solana_program::pubkey::Pubkey,
    accounts: &[solana_program::account_info::AccountInfo],
    data: &[u8],
)-> solana_program::entrypoint::ProgramResult{
    let accounts_iter = &mut accounts.iter();
    
}