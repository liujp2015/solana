const web3 = require('@solana/web3.js');

// 连接到本地 Solana 网络
const connection = new web3.Connection('http://127.0.0.1:8899', 'confirmed');

// 程序 ID
const programId = new web3.PublicKey('4iVfUS475Qna5uVVJ2o7PNt25Amo4VzbnM9Fc9Wq4ugd');

// 创建测试账户
const owner = web3.Keypair.generate();

async function testProgramExistence() {
  console.log('Testing program existence...');
  console.log(`Program ID: ${programId.toBase58()}`);
  console.log(`Owner: ${owner.publicKey.toBase58()}`);
  
  try {
    // 获取程序账户信息
    const programAccount = await connection.getAccountInfo(programId);
    
    if (programAccount) {
      console.log('✓ Program exists!');
      console.log(`Program data length: ${programAccount.data.length} bytes`);
      console.log(`Program owner: ${programAccount.owner.toBase58()}`);
      console.log(`Program executable: ${programAccount.executable}`);
    } else {
      console.log('✗ Program does not exist');
      return;
    }
    
    // 请求空投给 owner
    console.log('\nRequesting airdrop to owner...');
    const airdropSignature = await connection.requestAirdrop(
      owner.publicKey,
      web3.LAMPORTS_PER_SOL * 1
    );
    await connection.confirmTransaction(airdropSignature);
    
    // 检查余额
    const ownerBalance = await connection.getBalance(owner.publicKey);
    console.log(`Owner balance: ${ownerBalance / web3.LAMPORTS_PER_SOL} SOL`);
    
    // 生成 vault 地址
    const [vault, bump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('vault'), owner.publicKey.toBuffer()],
      programId
    );
    console.log(`\nVault address: ${vault.toBase58()}`);
    console.log(`Bump: ${bump}`);
    
    // 检查 vault 账户是否存在
    const vaultAccount = await connection.getAccountInfo(vault);
    if (vaultAccount) {
      console.log('✓ Vault account exists!');
      console.log(`Vault balance: ${vaultAccount.lamports / web3.LAMPORTS_PER_SOL} SOL`);
    } else {
      console.log('✗ Vault account does not exist (this is expected for a new vault)');
    }
    
    console.log('\nTest completed successfully!');
    
  } catch (error) {
    console.error('Error testing program:', error);
  }
}

testProgramExistence();
