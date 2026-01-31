#!/bin/bash

# 测试 withdraw 指令的脚本

# 设置变量
PROGRAM_ID="22222222222222222222222222222222222222222222"
OWNER_KEYPAIR="~/.config/solana/id.json"
VAULT_ADDRESS=""
AMOUNT=100000

# 生成 owner 密钥对（如果不存在）
if [ ! -f "$OWNER_KEYPAIR" ]; then
    echo "生成 owner 密钥对..."
    solana-keygen new --no-bip39-passphrase
fi

# 获取 owner 地址
OWNER_ADDRESS=$(solana-keygen pubkey "$OWNER_KEYPAIR")
echo "Owner 地址: $OWNER_ADDRESS"

# 计算 vault 地址
echo "计算 vault 地址..."
VAULT_ADDRESS=$(solana find-program-derived-address --program-id "$PROGRAM_ID" "vault" "$OWNER_ADDRESS")
echo "Vault 地址: $VAULT_ADDRESS"

# 部署程序
echo "部署程序..."
solana program deploy target/deploy/blueshift_vault.so

# 测试 withdraw 指令
echo "测试 withdraw 指令..."
# 构建指令数据：[1] + amount.to_le_bytes()
DISCRIMINATOR=1
AMOUNT_BYTES=$(python3 -c "import struct; print(''.join(['%02x' % b for b in struct.pack('<Q', $AMOUNT)]))")
INSTRUCTION_DATA=$(echo "$DISCRIMINATOR$AMOUNT_BYTES" | sed 's/^/0x/')

echo "指令数据: $INSTRUCTION_DATA"

# 系统程序 ID（硬编码）
SYSTEM_PROGRAM_ID="11111111111111111111111111111111"
echo "系统程序 ID: $SYSTEM_PROGRAM_ID"

# 创建一个简单的 Python 脚本来构建和发送交易
echo "创建测试交易脚本..."
cat > test_transaction.py << EOF
from solana.rpc.api import Client
from solana.keypair import Keypair
from solana.transaction import Transaction, TransactionInstruction
from solana.publickey import PublicKey
import base58

# 设置变量
program_id = PublicKey("$PROGRAM_ID")
owner_keypair = Keypair.from_secret_key(base58.b58decode(open("$OWNER_KEYPAIR").read()))
owner_address = owner_keypair.public_key
vault_address = PublicKey("$VAULT_ADDRESS")
system_program_id = PublicKey("$SYSTEM_PROGRAM_ID")
instruction_data = bytes.fromhex("$INSTRUCTION_DATA".replace("0x", ""))

# 创建交易
print("创建交易...")
tx = Transaction()
tx.add(
    TransactionInstruction(
        keys=[
            (owner_address, True),  # owner 账户，需要签名
            (vault_address, False),  # vault 账户，不需要签名
            (system_program_id, False),  # 系统程序账户，不需要签名
        ],
        program_id=program_id,
        data=instruction_data,
    )
)

# 发送交易
print("发送交易...")
client = Client("http://localhost:8899")
signature = client.send_transaction(tx, owner_keypair)
print(f"交易签名: {signature['result']}")

# 确认交易
print("确认交易...")
result = client.confirm_transaction(signature["result"])
print(f"交易结果: {result}")
EOF

# 运行测试交易脚本
echo "运行测试交易脚本..."
python3 test_transaction.py
