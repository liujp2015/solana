use pinocchio::{AccountView, Address, ProgramResult, cpi::{Seed, Signer}, error::ProgramError};
use pinocchio_system::instructions::Transfer;


pub struct VaultContext<'info>{
    owner:&'info AccountView,
    vault:&'info AccountView,
    lamports:u64,
    bump:u8
}

impl<'info> TryFrom<(&'info [AccountView], &'info [u8])> for VaultContext<'info>{
   type Error = ProgramError;
   fn try_from(value: (&'info [AccountView], &'info [u8])) -> Result<Self, Self::Error> {
     // Check account count
     if value.0.len() < 3 {
         return Err(ProgramError::NotEnoughAccountKeys);
     }
     
     let owner = &value.0[0];
     let vault = &value.0[1];
     
     // Checks owner is a signer
     if !owner.is_signer(){
        return Err(ProgramError::MissingRequiredSignature);
     }
     // check valult belongs to System Program
     if !vault.owned_by(&pinocchio_system::ID){
        return Err(ProgramError::InvalidAccountOwner);
     }
     
     // check vault address matchs
     let (vault_address,bump) = Address::find_program_address(&[b"vault",owner.address().as_ref()],&crate::ID);
     
     if vault_address.ne(vault.address()) {
        return Err(ProgramError::InvalidAccountData);
     }
     // 处理指令数据长度不足的情况
     let lamports = if value.1.len() == 8 {
         // 正常情况：8 字节金额数据
         let mut bytes = [0u8; 8];
         bytes.copy_from_slice(value.1);
         u64::from_le_bytes(bytes)
     } else {
         // 没有提供金额数据：提取所有余额
         vault.lamports()
     };
     Ok(Self { owner, vault, lamports,bump })
   }
}

impl<'info> VaultContext<'info>{
    pub fn deposit(&self) -> ProgramResult {
       Transfer {
        from:self.owner,
        to:self.vault,
        lamports:self.lamports
       }.invoke()?;
       Ok(())
    }
    pub fn withdraw(&self) -> ProgramResult {
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
        }.invoke_signed(&signers)?;
        Ok(())
     }
}
// cargo  add solana-program-log 