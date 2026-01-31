use blueshift_vault::process_instruction;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction, pubkey::Pubkey};
use std::str::FromStr;

#[tokio::test]
async fn test_deposit_instruction() {
    // Setup test environment
    let program_id = Pubkey::from_str("22222222222222222222222222222222222222222222").unwrap();
    let owner = Keypair::new();
    let vault = Keypair::new();
    
    // Create test accounts
    let mut program_test = ProgramTest::new(
        "blueshift_vault",
        program_id,
        processor!(process_instruction),
    );
    
    // Add accounts to test environment
    program_test.add_account(
        owner.pubkey(),
        solana_sdk::account::Account {
            lamports: 1000000,
            data: vec![],
            owner: solana_sdk::system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    );
    
    program_test.add_account(
        vault.pubkey(),
        solana_sdk::account::Account {
            lamports: 0,
            data: vec![],
            owner: solana_sdk::system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    );
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
    // Test deposit instruction
    let deposit_amount: u64 = 100000;
    let mut instruction_data = vec![0]; // Discriminator for deposit
    instruction_data.extend_from_slice(&deposit_amount.to_le_bytes());
    
    let instruction = solana_sdk::instruction::Instruction {
        program_id: program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(owner.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(vault.pubkey(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
        ],
        data: instruction_data,
    };
    
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer, &owner],
        recent_blockhash,
    );
    
    // Send transaction and check result
    let result = banks_client.process_transaction(transaction).await;
    println!("Deposit transaction result: {:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_withdraw_instruction() {
    // Setup test environment
    let program_id = Pubkey::from_str("22222222222222222222222222222222222222222222").unwrap();
    let owner = Keypair::new();
    let vault = Keypair::new();
    
    // Create test accounts
    let mut program_test = ProgramTest::new(
        "blueshift_vault",
        program_id,
        processor!(process_instruction),
    );
    
    // Add accounts to test environment
    program_test.add_account(
        owner.pubkey(),
        solana_sdk::account::Account {
            lamports: 500000,
            data: vec![],
            owner: solana_sdk::system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    );
    
    program_test.add_account(
        vault.pubkey(),
        solana_sdk::account::Account {
            lamports: 100000,
            data: vec![],
            owner: solana_sdk::system_program::ID,
            executable: false,
            rent_epoch: 0,
        },
    );
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
    // Test withdraw instruction
    let withdraw_amount: u64 = 50000;
    let mut instruction_data = vec![1]; // Discriminator for withdraw
    instruction_data.extend_from_slice(&withdraw_amount.to_le_bytes());
    
    let instruction = solana_sdk::instruction::Instruction {
        program_id: program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(owner.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(vault.pubkey(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
        ],
        data: instruction_data,
    };
    
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer, &owner],
        recent_blockhash,
    );
    
    // Send transaction and check result
    let result = banks_client.process_transaction(transaction).await;
    println!("Withdraw transaction result: {:?}", result);
    assert!(result.is_ok());
}
