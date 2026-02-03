use pinocchio::{
    AccountView, Address, ProgramResult,
};
use solana_program::program_error::ProgramError;
use solana_program::sysvar::Sysvar;
use core::mem::size_of;
use solana_program::pubkey::Pubkey;

// --- 账户结构 ---
pub struct MakeAccounts<'a> {
    pub maker: &'a AccountView,
    pub escrow: &'a AccountView,
    pub mint_a: &'a AccountView,
    pub mint_b: &'a AccountView,
    pub maker_ata_a: &'a AccountView,
    pub vault: &'a AccountView,
    pub system_program: &'a AccountView,
    pub token_program: &'a AccountView,
    pub associated_token_program: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for MakeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [maker, escrow, mint_a, mint_b, maker_ata_a, vault, system_program, token_program, associated_token_program] =
            accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };

        // 验证逻辑...
        crate::instructions::helpers::validate_maker_account(maker)?;
        crate::instructions::helpers::validate_mint_account(mint_a)?;
        crate::instructions::helpers::validate_mint_interface(mint_b)?;
        crate::instructions::helpers::validate_associated_token_account(maker_ata_a, maker, mint_a, token_program)?;
        crate::instructions::helpers::validate_system_program(system_program)?;
        crate::instructions::helpers::validate_token_program(token_program)?;

        if associated_token_program.address().as_ref() != spl_associated_token_account::ID.as_ref() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self {
            maker,
            escrow,
            mint_a,
            mint_b,
            maker_ata_a,
            vault,
            system_program,
            token_program,
            associated_token_program,
        })
    }
}

// --- 指令数据 ---
pub struct MakeInstructionData {
    pub seed: u64,
    pub receive: u64,
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for MakeInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() * 3 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let seed = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let receive = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let amount = u64::from_le_bytes(data[16..24].try_into().unwrap());

        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self { seed, receive, amount })
    }
}

// --- 主指令上下文 ---
pub struct Make<'a> {
    pub accounts: MakeAccounts<'a>,
    pub instruction_data: MakeInstructionData,
    pub bump: u8,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for Make<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
        let accounts = MakeAccounts::try_from(accounts)?;
        let instruction_data = MakeInstructionData::try_from(data)?;

        // 计算预期的 escrow PDA 地址
        let (expected_escrow, bump) = Address::find_program_address(
            &[
                b"escrow",
                accounts.maker.address().as_ref(),
                &instruction_data.seed.to_le_bytes(),
            ],
            &crate::ID,
        );

        // 验证传入的 escrow 账户地址匹配
        if accounts.escrow.address() != &expected_escrow {
            return Err(ProgramError::InvalidSeeds);
        }

        // 检查 escrow 是否未初始化
        if !accounts.escrow.is_data_empty() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // === 1. 初始化 Escrow 账户（PDA）===
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(crate::Escrow::LEN);

        // 构造系统指令：注意 Pubkey 转换方式
        let create_escrow_ix = solana_program::system_instruction::create_account(
            &Pubkey::new_from_array(*accounts.maker.address()),
            &Pubkey::new_from_array(*accounts.escrow.address()),
            lamports,
            crate::Escrow::LEN as u64,
            &Pubkey::new_from_array(crate::ID.to_bytes()),
        );

        // 使用 pinocchio-system 来 invoke，避免 AccountInfo 转换问题
        pinocchio_system::invoke(
            &create_escrow_ix,
            &[
                accounts.maker.into(),
                accounts.escrow.into(),
                accounts.system_program.into(),
            ],
        )?;

        // === 2. 初始化 Vault（Associated Token Account）===
        // 使用 pinocchio-associated-token-account 提供的 helper
        pinocchio_associated_token_account::create_associated_token_account(
            accounts.maker,
            accounts.escrow,
            accounts.mint_a,
        )?;

        Ok(Self {
            accounts,
            instruction_data,
            bump,
        })
    }
}

impl<'a> Make<'a> {
    pub const DISCRIMINATOR: u8 = 0;

    pub fn process(&mut self) -> ProgramResult {
        // 写入 Escrow 数据
        let mut data = self.accounts.escrow.try_borrow_mut_data()?;
        let escrow = crate::Escrow::load_mut(data.as_mut())?;
        escrow.set_inner(
            self.instruction_data.seed,
            *self.accounts.maker.address(),   // ✅ .address()
            *self.accounts.mint_a.address(),
            *self.accounts.mint_b.address(),
            self.instruction_data.receive,
            [self.bump],
        );

        // 从 maker_ata_a 转账到 vault
        pinocchio_token::Transfer {
            from: self.accounts.maker_ata_a,
            to: self.accounts.vault,
            authority: self.accounts.maker,
            amount: self.instruction_data.amount,
        }.invoke()?;

        Ok(())
    }
}

// 显式导入 Rent（避免未使用警告）
use solana_program::sysvar::rent::Rent;