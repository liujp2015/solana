const web3 = require('@solana/web3.js');
const fs = require('fs');
const path = require('path');

// 连接到本地 Solana 网络
const connection = new web3.Connection('http://127.0.0.1:8899', 'confirmed');

// 使用新部署的程序ID
const programId = new web3.PublicKey('Hw17uR386oyYJoXR9EdpKFyG18CjUmHEp6fxLgbjAo2');

// 创建测试账户
const payer = web3.Keypair.generate();
const owner = web3.Keypair.generate();

async function testProgram() {
  // 使用 createAccountWithSeed 派生 vault 地址
  const seed = 'vault';
  const vault = await web3.PublicKey.createWithSeed(owner.publicKey, seed, programId);
  
  console.log('Testing Pinocchio Test Program...');
  console.log(`Program ID: ${programId.toBase58()}`);
  console.log(`Payer: ${payer.publicKey.toBase58()}`);
  console.log(`Owner: ${owner.publicKey.toBase58()}`);
  console.log(`Vault: ${vault.toBase58()}`);
  
  // 请求空投给 payer
  console.log('\nRequesting airdrop to payer...');
  const airdropSignature = await connection.requestAirdrop(
    payer.publicKey,
    web3.LAMPORTS_PER_SOL * 2
  );
  await connection.confirmTransaction(airdropSignature);
  
  // 请求空投给 owner - 给更多的余额
  console.log('Requesting airdrop to owner...');
  const airdropSignature2 = await connection.requestAirdrop(
    owner.publicKey,
    web3.LAMPORTS_PER_SOL * 2
  );
  await connection.confirmTransaction(airdropSignature2);
  
  // 检查余额
  const payerBalance = await connection.getBalance(payer.publicKey);
  const ownerBalance = await connection.getBalance(owner.publicKey);
  console.log(`\nInitial balances:`);
  console.log(`Payer: ${payerBalance / web3.LAMPORTS_PER_SOL} SOL`);
  console.log(`Owner: ${ownerBalance / web3.LAMPORTS_PER_SOL} SOL`);
  
  // 创建 vault 账户 - 使用 createAccountWithSeed
  console.log('\nCreating vault account...');
  const rentExemptAmount = await connection.getMinimumBalanceForRentExemption(0);
  
  const createVaultInstruction = web3.SystemProgram.createAccountWithSeed({
    fromPubkey: payer.publicKey,
    newAccountPubkey: vault,
    basePubkey: owner.publicKey,
    seed: seed,
    lamports: rentExemptAmount,
    space: 0,
    programId: programId
  });
  
  const createVaultTransaction = new web3.Transaction().add(createVaultInstruction);
  const createVaultSignature = await web3.sendAndConfirmTransaction(
    connection,
    createVaultTransaction,
    [payer, owner]
  );
  console.log(`Vault account created: ${createVaultSignature}`);
  
  // 测试存款
  console.log('\nTesting deposit...');
  const depositAmount = Math.floor(web3.LAMPORTS_PER_SOL * 0.5);
  console.log(`Deposit amount: ${depositAmount} lamports (${depositAmount / web3.LAMPORTS_PER_SOL} SOL)`);
  
  // 1. 首先使用 System Program 向 vault 账户转账
  console.log('Transferring SOL to vault...');
  const transferInstruction = web3.SystemProgram.transfer({
    fromPubkey: owner.publicKey,
    toPubkey: vault,
    lamports: depositAmount
  });
  
  // 2. 然后调用我们的程序进行验证
  // 将金额转换为8字节的字节数组（小端序）
  const amountBuffer = Buffer.alloc(8);
  amountBuffer.writeBigUInt64LE(BigInt(depositAmount));
  
  const depositInstruction = new web3.TransactionInstruction({
    keys: [
      {
        pubkey: vault,
        isSigner: false,
        isWritable: true
      },
      {
        pubkey: owner.publicKey,
        isSigner: true,
        isWritable: false
      }
    ],
    programId,
    data: Buffer.concat([
      Buffer.from([0]), // Deposit discriminator
      amountBuffer // lamports (8 bytes, little-endian)
    ])
  });
  
  // 组合两个指令到一个交易中
  const depositTransaction = new web3.Transaction()
    .add(transferInstruction)
    .add(depositInstruction);
  
  try {
    const depositSignature = await web3.sendAndConfirmTransaction(
      connection,
      depositTransaction,
      [owner]
    );
    console.log(`Deposit transaction signature: ${depositSignature}`);
  } catch (error) {
    console.error('Deposit error details:', error);
    if (error.getLogs) {
      console.error('Deposit logs:', await error.getLogs());
    }
    throw error;
  }
  
  // 检查余额
  const ownerBalanceAfterDeposit = await connection.getBalance(owner.publicKey);
  const vaultBalanceAfterDeposit = await connection.getBalance(vault);
  console.log(`\nBalances after deposit:`);
  console.log(`Owner: ${ownerBalanceAfterDeposit / web3.LAMPORTS_PER_SOL} SOL`);
  console.log(`Vault: ${vaultBalanceAfterDeposit / web3.LAMPORTS_PER_SOL} SOL`);
  
  // 测试取款
  console.log('\nTesting withdraw...');
  
  const withdrawInstruction = new web3.TransactionInstruction({
    keys: [
      {
        pubkey: vault,
        isSigner: false,
        isWritable: true
      },
      {
        pubkey: owner.publicKey,
        isSigner: true,
        isWritable: true
      }
    ],
    programId,
    data: Buffer.from([1]) // Withdraw discriminator
  });
  
  const withdrawTransaction = new web3.Transaction().add(withdrawInstruction);
  try {
    const withdrawSignature = await web3.sendAndConfirmTransaction(
      connection,
      withdrawTransaction,
      [owner]
    );
    console.log(`Withdraw transaction signature: ${withdrawSignature}`);
  } catch (error) {
    console.error('Withdraw error details:', error);
    if (error.getLogs) {
      console.error('Withdraw logs:', await error.getLogs());
    }
    throw error;
  }
  
  // 检查余额
  const ownerBalanceAfterWithdraw = await connection.getBalance(owner.publicKey);
  const vaultBalanceAfterWithdraw = await connection.getBalance(vault);
  console.log(`\nBalances after withdraw:`);
  console.log(`Owner: ${ownerBalanceAfterWithdraw / web3.LAMPORTS_PER_SOL} SOL`);
  console.log(`Vault: ${vaultBalanceAfterWithdraw / web3.LAMPORTS_PER_SOL} SOL`);
  
  console.log('\nTest completed successfully!');
}

testProgram().catch(err => {
  console.error('Error testing program:', err);
  process.exit(1);
});
