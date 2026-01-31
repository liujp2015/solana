#![no_std]

use pinocchio::{AccountView, Address, entrypoint, nostd_panic_handler, ProgramResult};
use pinocchio::error::ProgramError;

// 类型别名，保持变量名不变
type AccountInfo = AccountView;
type Pubkey = Address;

// 导入已有的 instruction 模块
mod instruction;
use instruction::Deposit;
use instruction::Withdraw;

entrypoint!(process_instruction);
nostd_panic_handler!();

// 22222222222222222222222222222222222222222
pub const ID: Pubkey = Address::new_from_array([
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07,
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07,
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
]);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((discriminator, data)) if *discriminator == Deposit::Deposit::DISCRIMINATOR => {
            let mut deposit = Deposit::Deposit::try_from((data, accounts))?;
            deposit.process()
        },
        Some((discriminator, _)) if *discriminator == Withdraw::Withdraw::DISCRIMINATOR => {
            let mut withdraw = Withdraw::Withdraw::try_from(accounts)?;
            withdraw.process()
        },
        _ => Err(ProgramError::InvalidInstructionData),
    }
}