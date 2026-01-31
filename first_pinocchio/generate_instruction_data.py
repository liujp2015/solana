#!/usr/bin/env python3

# 生成正确的 withdraw 指令数据格式

import struct

# 设置变量
discriminator = 1  # withdraw 指令的 discriminator
amount = 100000  # 测试金额

# 构建指令数据：[discriminator] + amount.to_le_bytes()
amount_bytes = struct.pack('<Q', amount)
instruction_data = bytes([discriminator]) + amount_bytes

print(f"正确的 withdraw 指令数据格式:")
print(f"总长度: {len(instruction_data)} 字节")
print(f"十六进制: {instruction_data.hex()}")
print(f"二进制: {[b for b in instruction_data]}")
print(f"\n说明:")
print(f"- 第一个字节 {discriminator} 是 withdraw 指令的 discriminator")
print(f"- 接下来的 8 字节 {amount_bytes.hex()} 是金额 {amount} 的小端字节序表示")
print(f"\n使用方法:")
print(f"在构建交易时，将上述指令数据作为 instruction.data 字段的值")
