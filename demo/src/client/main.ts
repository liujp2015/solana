import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from '@solana/web3.js';
import * as fs from 'fs';

// ===== 配置 =====
const PROGRAM_ID = new PublicKey('JB7aEdWTn7q3rnS1RAVGZa7HD17KWpeV1hrTjeSkDWvD');
const CLUSTER_URL = 'https://solana-devnet.g.alchemy.com/v2/kxV4RkX7OkCJ22nzAUUaj'; // 或 http://localhost:8899 用于 localnet   https://api.devnet.solana.com 
const connection = new Connection(CLUSTER_URL, 'confirmed');

// 读取 CLI 默认钱包（仅用于测试！）
function getTestKeypair(): Keypair {
  try {
    const homeDir = require('os').homedir();
    const keypairPath = `${homeDir}/.config/solana/id.json`;
    const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync(keypairPath, 'utf8')));
    return Keypair.fromSecretKey(secretKey);
  } catch (err) {
    console.error('❌ 请先配置 Solana CLI 钱包: solana-keygen new');
    process.exit(1);
  }
}

async function saveData() {
  const user = getTestKeypair();
  console.log('Using wallet:', user.publicKey.toBase58());

  // 派生 PDA —— ⚠️ 请根据你的 Rust 程序逻辑修改 seeds！
  const [dataPda] = PublicKey.findProgramAddressSync(
    [user.publicKey.toBuffer(), Buffer.from('data')], // 常见 seed 格式
    PROGRAM_ID
  );

  console.log('PDA address:', dataPda.toBase58());

  // 指令数据
  const instructionData = Buffer.from('Hello from TypeScript!');

  // 构造 accounts
  const keys = [
    { pubkey: user.publicKey, isSigner: true, isWritable: true },
    { pubkey: dataPda, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
  ];

  const instruction = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys,
    data: instructionData,
  });

  // 发送交易
  const tx = new Transaction().add(instruction);
  tx.feePayer = user.publicKey;
  const { blockhash } = await connection.getLatestBlockhash();
  tx.recentBlockhash = blockhash;

  try {
    const txid = await sendAndConfirmTransaction(connection, tx, [user]);
    console.log('✅ Success! TX:', txid);
    console.log(`🔗 https://explorer.solana.com/tx/${txid}?cluster=devnet`);
  } catch (err: any) {
    console.error('❌ Transaction failed:');
    console.error(err.message);
    if (err.logs) console.error('Logs:', err.logs);
    process.exit(1);
  }
 
}

// 执行
saveData().catch((err) => {
  console.error('Test failed:', err);
  process.exit(1);
});