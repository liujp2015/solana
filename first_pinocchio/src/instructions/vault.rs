use pinocchio::{AccountView, Address, ProgramResult, cpi::{Seed, Signer}, error::ProgramError};
use pinocchio_system::instructions::Transfer;
use solana_program::{log, program_error};
use solana_program_log::log;


pub struct VaultContext<'info>{
    owner:&'info AccountView,
    vault:&'info AccountView,
    lamports:u64,
    bump:u8
}

impl<'info> TryFrom<(&'info [AccountView], &'info [u8])> for VaultContext<'info>{
   type Error = ProgramError;
   fn try_from(value: (&'info [AccountView], &'info [u8])) -> Result<Self, Self::Error> {
     let [owner,vault,_] = value.0 else {
         return Err(ProgramError::NotEnoughAccountKeys);
     };
     // Checks owner is a signer
     if !owner.is_signer(){
        return Err(ProgramError::MissingRequiredSignature);
     }
     // check valult belongs to System Program
     if !vault.owned_by(&pinocchio_system::ID){
        return Err(ProgramError::InvalidAccountOwner);
     }
     // check vault address matchs
     let (vault_address,bump) = Address::find_program_address(&[b"vault'",owner.address().as_ref()],&crate::ID);
     
     if vault_address.ne(vault.address()) {
        return Err(ProgramError::InvalidAccountData);
     }
     // check amount is correct length
     if value.1.len()!= core::mem::size_of::<u64>(){
        return  Err(ProgramError::InvalidInstructionData);
     }
     let lamports:u64 = u64::from_le_bytes(value.1.try_into().unwrap());
     Ok({Self { owner, vault, lamports,bump }})
     //cargo add solana-address
   }
}  

impl<'info> VaultContext<'info>{
    pub fn deposit(&self) -> ProgramResult {
       log("Deposit Invoke");
       Transfer {
        from:self.owner,
        to:self.vault,
        lamports:self.lamports
       }.invoke();
       Ok(())
    }
    pub fn withdraw(&self) -> ProgramResult {
        log("Withdraw Invoke");
        let bump = [self.bump];
        let seeds = [
         Seed::from(b"vault"),
         Seed::from(self.owner.address().as_ref()),
         Seed::from(bump.as_ref())];

         let signers = [Signer::from(&seeds)];
        Transfer {
         from:self.vault,
         to:self.owner,
         lamports:self.lamports
        }.invoke_signed(&signers);
        Ok(())
     }
}
// cargo  add solana-program-log 