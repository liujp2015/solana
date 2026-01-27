#!/bin/bash

# 设置程序 ID
PROGRAM_ID="4iVfUS475Qna5uVVJ2o7PNt25Amo4VzbnM9Fc9Wq4ugd"

# 生成测试账户
OWNER_KEYPAIR="owner-keypair.json"
if [ ! -f "$OWNER_KEYPAIR" ]; then
    echo "Generating owner keypair..."
    solana-keygen new --outfile "$OWNER_KEYPAIR" --no-passphrase
fi

OWNER_PUBKEY=$(solana-keygen pubkey "$OWNER_KEYPAIR")

# 检查程序状态
echo "\n=== Testing Pinocchio Test Program ==="
echo "Program ID: $PROGRAM_ID"
echo "Owner: $OWNER_PUBKEY"

# 检查程序账户
echo "\n=== Checking Program Account ==="
solana account "$PROGRAM_ID"

# 请求空投给 owner
echo "\n=== Requesting Airdrop ==="
solana airdrop 2 "$OWNER_PUBKEY" --url http://127.0.0.1:8899

# 检查余额
echo "\n=== Checking Balances ==="
solana balance "$OWNER_PUBKEY" --url http://127.0.0.1:8899

# 生成 vault 地址
echo "\n=== Generating Vault Address ==="
VAULT_ADDRESS=$(solana address --program-id "$PROGRAM_ID" --keypair "$OWNER_KEYPAIR" "vault")
echo "Vault address: $VAULT_ADDRESS"

# 检查 vault 账户
echo "\n=== Checking Vault Account ==="
solana account "$VAULT_ADDRESS" 2>/dev/null || echo "Vault account does not exist (this is expected for a new vault)"

echo "\n=== Test Completed ==="
