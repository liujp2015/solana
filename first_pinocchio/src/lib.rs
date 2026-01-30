use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};
use solana_address::declare_id;

use crate::instructions::Deposit;

entrypoint!(process_instruction);

mod instructions; 

declare_id!("22222222222222222222222222222222222222222222");

fn process_instruction(
    program_id: &Address,      // Address of the account the program was loaded into
    accounts: &[AccountView], // All accounts required to process the instruction
    instruction_data: &[u8],  // Serialized instruction-specific data
) -> ProgramResult{
    let (discriminator, instruction_data): (&u8, &[u8]) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    match  *discriminator {
        0 => Deposit::try_from((accounts,instruction_data))?.process(),
        1 => Ok(()),
        _ => Err(ProgramError::InvalidInstructionData)
    }
   
}