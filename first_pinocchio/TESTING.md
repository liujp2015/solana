# Solana 程序测试指南

本文档提供了在本地环境中测试 Solana 程序的详细步骤。

## 1. 安装必要的工具

确保你已经安装了以下工具：

### 1.1 Solana CLI 工具

```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.18.26/install)"
```

### 1.2 Rust 编译器

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 1.3 构建工具

```bash
cargo install cargo-bpf
```

## 2. 构建程序

在项目目录中运行以下命令来构建程序：

```bash
cargo build-sbf
```

这会生成一个 `.so` 文件，位于 `target/deploy/blueshift_vault.so`。

## 3. 启动本地测试网络

运行以下命令启动一个本地的 Solana 测试网络：

```bash
solana-test-validator
```

## 4. 部署程序

在另一个终端中，运行以下命令来部署你的程序：

```bash
solana program deploy target/deploy/blueshift_vault.so
```

部署成功后，你会得到一个程序 ID，类似于：

```
Program Id: 22222222222222222222222222222222222222222222
```

## 5. 测试程序

### 5.1 创建测试账户

```bash
solana-keygen new --no-bip39-passphrase -o owner.json
solana-keygen new --no-bip39-passphrase -o vault.json
```

### 5.2 为测试账户充值

```bash
solana airdrop 1 $(solana-keygen pubkey owner.json)
```

### 5.3 使用 JavaScript 测试

创建一个 `test.js` 文件，内容如下：

```javascript
const web3 = require('@solana/web3.js');
const fs = require('fs');

// 配置
const programId = new web3.PublicKey('YOUR_PROGRAM_ID');
const ownerKeyPair = web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync('owner.json', 'utf8')))
);
const vaultKeyPair = web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync('vault.json', 'utf8')))
);
const connection = new web3.Connection(web3.clusterApiUrl('localhost'), 'confirmed');

// 构建 deposit 交易
async function testDeposit() {
    console.log('测试 deposit 指令...');
    
    const transaction = new web3.Transaction();
    
    // 构建 deposit 指令数据：[0] + amount.to_le_bytes()
    const amount = 100000;
    const instructionData = Buffer.alloc(9);
    instructionData[0] = 0; // deposit discriminator
    instructionData.writeUIntLE(amount, 1, 8);
    
    const instruction = new web3.TransactionInstruction({
        keys: [
            { pubkey: ownerKeyPair.publicKey, isSigner: true, isWritable: true },
            { pubkey: vaultKeyPair.publicKey, isSigner: false, isWritable: true },
            { pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data: instructionData,
    });
    
    transaction.add(instruction);
    
    // 发送交易
    const signature = await web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [ownerKeyPair]
    );
    
    console.log(`Deposit 交易签名: ${signature}`);
    
    // 检查账户余额
    const ownerBalance = await connection.getBalance(ownerKeyPair.publicKey);
    const vaultBalance = await connection.getBalance(vaultKeyPair.publicKey);
    console.log(`Owner 余额: ${ownerBalance}`);
    console.log(`Vault 余额: ${vaultBalance}`);
}

// 构建 withdraw 交易
async function testWithdraw() {
    console.log('\n测试 withdraw 指令...');
    
    const transaction = new web3.Transaction();
    
    // 构建 withdraw 指令数据：[1] + amount.to_le_bytes()
    const amount = 50000;
    const instructionData = Buffer.alloc(9);
    instructionData[0] = 1; // withdraw discriminator
    instructionData.writeUIntLE(amount, 1, 8);
    
    const instruction = new web3.TransactionInstruction({
        keys: [
            { pubkey: ownerKeyPair.publicKey, isSigner: true, isWritable: true },
            { pubkey: vaultKeyPair.publicKey, isSigner: false, isWritable: true },
            { pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data: instructionData,
    });
    
    transaction.add(instruction);
    
    // 发送交易
    const signature = await web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [ownerKeyPair]
    );
    
    console.log(`Withdraw 交易签名: ${signature}`);
    
    // 检查账户余额
    const ownerBalance = await connection.getBalance(ownerKeyPair.publicKey);
    const vaultBalance = await connection.getBalance(vaultKeyPair.publicKey);
    console.log(`Owner 余额: ${ownerBalance}`);
    console.log(`Vault 余额: ${vaultBalance}`);
}

// 测试提取所有余额
async function testWithdrawAll() {
    console.log('\n测试提取所有余额...');
    
    const transaction = new web3.Transaction();
    
    // 构建 withdraw 指令数据：[1]（只有 discriminator，没有金额数据）
    const instructionData = Buffer.alloc(1);
    instructionData[0] = 1; // withdraw discriminator
    
    const instruction = new web3.TransactionInstruction({
        keys: [
            { pubkey: ownerKeyPair.publicKey, isSigner: true, isWritable: true },
            { pubkey: vaultKeyPair.publicKey, isSigner: false, isWritable: true },
            { pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data: instructionData,
    });
    
    transaction.add(instruction);
    
    // 发送交易
    const signature = await web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [ownerKeyPair]
    );
    
    console.log(`Withdraw All 交易签名: ${signature}`);
    
    // 检查账户余额
    const ownerBalance = await connection.getBalance(ownerKeyPair.publicKey);
    const vaultBalance = await connection.getBalance(vaultKeyPair.publicKey);
    console.log(`Owner 余额: ${ownerBalance}`);
    console.log(`Vault 余额: ${vaultBalance}`);
}

// 运行测试
testDeposit()
    .then(() => testWithdraw())
    .then(() => testWithdrawAll())
    .catch(err => console.error(err));
```

### 5.4 安装依赖并运行测试

```bash
npm init -y
npm install @solana/web3.js
node test.js
```

## 6. 查看交易日志

使用以下命令查看交易的执行日志：

```bash
solana logs
```

这会显示交易的执行过程和任何日志输出，帮助你诊断问题。

## 7. 清理测试网络

当你完成测试后，可以停止本地测试网络：

```bash
pkill -f solana-test-validator
```

## 8. 常见问题

### 8.1 账户余额不足

如果遇到账户余额不足的问题，可以使用以下命令为账户充值：

```bash
solana airdrop 1 $(solana-keygen pubkey owner.json)
```

### 8.2 程序部署失败

如果程序部署失败，可能是因为：
- 本地测试网络没有启动
- 账户余额不足
- 程序代码有错误

### 8.3 交易执行失败

如果交易执行失败，查看交易日志以获取详细信息：

```bash
solana logs
```

## 9. 测试数据格式

### 9.1 deposit 指令数据

- **总长度**：9 字节
- **格式**：`[0, amount_1, amount_2, ..., amount_8]`
- **说明**：
  - 第一个字节 `0` 是 deposit 指令的 discriminator
  - 接下来的 8 字节是金额的小端字节序表示

### 9.2 withdraw 指令数据

- **总长度**：9 字节或 1 字节
- **格式 1**：`[1, amount_1, amount_2, ..., amount_8]`（指定金额）
- **格式 2**：`[1]`（提取所有余额）
- **说明**：
  - 第一个字节 `1` 是 withdraw 指令的 discriminator
  - 当提供金额数据时，接下来的 8 字节是金额的小端字节序表示
  - 当没有提供金额数据时，程序会提取 vault 账户中的所有余额

## 10. 示例输出

### 10.1 deposit 指令

```
测试 deposit 指令...
Deposit 交易签名: 5xxxxx...
Owner 余额: 900000
Vault 余额: 100000
```

### 10.2 withdraw 指令

```
测试 withdraw 指令...
Withdraw 交易签名: 6xxxxx...
Owner 余额: 950000
Vault 余额: 50000
```

### 10.3 提取所有余额

```
测试提取所有余额...
Withdraw All 交易签名: 7xxxxx...
Owner 余额: 1000000
Vault 余额: 0
```

---

通过以上步骤，你可以在本地环境中成功测试 Solana 程序的功能。如果遇到任何问题，请查看交易日志以获取详细信息。
