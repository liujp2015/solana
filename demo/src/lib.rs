#![allow(unexpected_cfgs)]

use solana_program::{account_info::{AccountInfo, next_account_info},entrypoint, entrypoint:: ProgramResult, program::invoke_signed, pubkey::Pubkey, rent::Rent, system_instruction::create_account, sysvar::Sysvar};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account_user = next_account_info(accounts_iter)?;
    let account_data = next_account_info(accounts_iter)?;
    let _ = next_account_info(accounts_iter)?; // Program system
    let _ = next_account_info(accounts_iter)?; // Program sysvar rent

    let rent_exemption = Rent::get()?.minimum_balance(data.len());
    let bump_seed = Pubkey::find_program_address(&[&account_user.key.to_bytes()], program_id).1;

    // Data account is not initialized. Create an account and write data into it.
    if **account_data.try_borrow_lamports().unwrap() == 0 {
       invoke_signed(
            &create_account(
                account_user.key,
                account_data.key,
                rent_exemption,
                data.len() as u64,
                program_id,
            ),
            accounts,
            &[&[&account_user.key.to_bytes(), &[bump_seed]]],
        )?;
        account_data.data.borrow_mut().copy_from_slice(data);
        return Ok(());
    }
    Ok(())
}
