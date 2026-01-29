use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};

entrypoint!(process_instruction);

mod instructions; 
fn process_instruction(
    program_id: &Address,      // Address of the account the program was loaded into
    accounts: &[AccountView], // All accounts required to process the instruction
    instruction_data: &[u8],  // Serialized instruction-specific data
) -> ProgramResult{
    let (discriminator, instruction_data): (&u8, &[u8]) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    match  *discriminator {
        0 => Ok(()),
        1 => Ok(()),
        _ => Err(ProgramError::InvalidInstructionData)
    }
   
}