use pinocchio::{AccountView, ProgramResult, error::ProgramError};
use solana_program_log::log;

pub struct Deposit<'info>{
    owner:&'info AccountView,
    vault:&'info AccountView,
    amount:u64,
}
impl<'info> Deposit<'info>{
    pub fn process(accounts:&'info[AccountView],instruction_data:&[u8]) -> ProgramResult {
        let  [owner,vault,_] = accounts else {
            return Err(pinocchio::error::ProgramError::NotEnoughAccountKeys);
        };
       if instruction_data.len()!=core::mem::size_of::<u64>(){
        return Err(ProgramError::InvalidInstructionData);
       }
       let amount:u64 = u64::from_le_bytes(instruction_data.try_into().unwrap());
       // let amount: u64 = u64::from_le_bytes(instruction_data.try_into().map(|_| ProgramError::InvalidInstructionData));
        log("Deposit Invoked");
        Ok(())
    }
}
// cargo  add solana-program-log 