#!/usr/bin/env python3

# 验证 withdraw 指令数据格式的脚本

import struct

# 测试 withdraw 指令数据
print("测试 withdraw 指令数据格式...")

# 设置变量
discriminator = 1  # withdraw 指令的 discriminator
amount = 100000  # 测试金额

# 构建指令数据：[discriminator] + amount.to_le_bytes()
amount_bytes = struct.pack('<Q', amount)
instruction_data = bytes([discriminator]) + amount_bytes

print(f"指令数据: {instruction_data.hex()}")
print(f"指令数据长度: {len(instruction_data)}")
print(f"Discriminator: {discriminator}")
print(f"Amount (小端字节序): {amount_bytes.hex()}")
print(f"Amount (十进制): {amount}")

# 验证指令数据格式
if len(instruction_data) != 9:
    print("错误: 指令数据长度应该是 9 字节 (1 字节 discriminator + 8 字节 amount)")
else:
    print("正确: 指令数据长度是 9 字节")

if instruction_data[0] != 1:
    print("错误: withdraw 指令的 discriminator 应该是 1")
else:
    print("正确: withdraw 指令的 discriminator 是 1")

# 解析指令数据
parsed_discriminator = instruction_data[0]
parsed_amount_bytes = instruction_data[1:9]
parsed_amount = struct.unpack('<Q', parsed_amount_bytes)[0]

print(f"\n解析结果:")
print(f"Discriminator: {parsed_discriminator}")
print(f"Amount (小端字节序): {parsed_amount_bytes.hex()}")
print(f"Amount (十进制): {parsed_amount}")

if parsed_amount == amount:
    print("正确: 解析的金额与原始金额匹配")
else:
    print("错误: 解析的金额与原始金额不匹配")
