// 告诉 Rust 不使用标准库（Solana 程序必须）
#![no_std]

// 引入 Pinocchio 的关键类型
use pinocchio::{
    account_info::AccountInfo,
    program_start,
};

// 定义错误码（可选，但推荐）
const ERROR_NOT_WRITABLE: u64 = 1;

// 入口函数：Solana 调用时会执行这个函数
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    // 解析输入：获取账户列表、指令数据、程序 ID
    let (accounts, _instruction_data, _program_id) = program_start!(input);

    // 我们假设第一个传入的账户是我们要操作的计数器账户
    let account = AccountInfo::from_ptr(accounts as *const u8);

    // 检查账户是否可写（否则不能修改）
    if !account.is_writable() {
        return ERROR_NOT_WRITABLE;
    }

    // 获取账户的数据（可变引用）
    let data = account.borrow_mut_data();

    // 确保数据至少有 1 字节
    if data.len() == 0 {
        return 100; // 自定义错误：数据太短
    }

    // 把第一个字节加 1（注意：最大到 255 会回绕）
    data[0] = data[0].wrapping_add(1);

    // 返回 0 表示成功
    0
}