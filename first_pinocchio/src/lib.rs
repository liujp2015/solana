use pinocchio::{AccountView, Address, ProgramResult, entrypoint};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Address,      // Address of the account the program was loaded into
    accounts: &[AccountView], // All accounts required to process the instruction
    instruction_data: &[u8],  // Serialized instruction-specific data
) -> ProgramResult{
    Ok(())
}