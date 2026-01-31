from solana.rpc.api import Client
from solana.keypair import Keypair
from solana.transaction import Transaction, TransactionInstruction
from solana.publickey import PublicKey
import base58

# 设置变量
program_id = PublicKey("22222222222222222222222222222222222222222222")
owner_keypair = Keypair.from_secret_key(base58.b58decode(open("~/.config/solana/id.json").read()))
owner_address = owner_keypair.public_key
vault_address = PublicKey("")
system_program_id = PublicKey("11111111111111111111111111111111")
instruction_data = bytes.fromhex("0x1a086010000000000".replace("0x", ""))

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
