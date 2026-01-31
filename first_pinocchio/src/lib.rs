use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};
use solana_address::declare_id;
use solana_program_log::log;

use crate::instructions::VaultContext;

entrypoint!(process_instruction);

mod instructions; 

declare_id!("22222222222222222222222222222222222222222222");

pub fn process_instruction(
    _program_id: &Address,      // Address of the account the program was loaded into
    accounts: &[AccountView], // All accounts required to process the instruction
    instruction_data: &[u8],  // Serialized instruction-specific data
) -> ProgramResult{
    log(&format!("Data len: {}", instruction_data.len()));
    log(&format!("Discriminator: {:?}", instruction_data.first()));
    let (discriminator, instruction_data): (&u8, &[u8]) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    log(&format!("Cmd: {}", discriminator));
    log(&format!("Amount len: {}", instruction_data.len()));
    match  *discriminator {
        0 => VaultContext::try_from((accounts,instruction_data))?.deposit(),
        1 => VaultContext::try_from((accounts,instruction_data))?.withdraw(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
   
}