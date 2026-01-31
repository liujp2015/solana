use pinocchio::{AccountView, Address, ProgramResult, error::ProgramError};
use crate::instructions::VaultContext;

fn main() {
    // 测试 VaultContext 的 TryFrom 实现
    println!("测试 VaultContext 的 TryFrom 实现...");
    
    // 创建模拟的账户数组
    let owner = AccountView::default();
    let vault = AccountView::default();
    let system_program = AccountView::default();
    let accounts = [owner, vault, system_program];
    
    // 测试 deposit 指令数据
    println!("测试 deposit 指令数据...");
    let deposit_amount: u64 = 100000;
    let deposit_data = deposit_amount.to_le_bytes();
    match VaultContext::try_from((&accounts, &deposit_data)) {
        Ok(vault_context) => println!("deposit 指令数据解析成功: {:?}", vault_context),
        Err(err) => println!("deposit 指令数据解析失败: {:?}", err),
    }
    
    // 测试 withdraw 指令数据
    println!("测试 withdraw 指令数据...");
    let withdraw_amount: u64 = 50000;
    let withdraw_data = withdraw_amount.to_le_bytes();
    match VaultContext::try_from((&accounts, &withdraw_data)) {
        Ok(vault_context) => println!("withdraw 指令数据解析成功: {:?}", vault_context),
        Err(err) => println!("withdraw 指令数据解析失败: {:?}", err),
    }
}

// 为了编译通过，我们需要添加一些默认实现
impl Default for AccountView {
    fn default() -> Self {
        // 这里返回一个默认的 AccountView
        // 实际测试中，我们需要提供真实的账户数据
        unsafe {
            std::mem::zeroed()
        }
    }
}
