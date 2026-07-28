#!/bin/bash
# 模块 5.1：什么是 API — curl 调用示例

echo "========================================"
echo "  用 curl 调用真实 API"
echo "========================================"

echo ""
echo "===== 1. 查询公网 IP（ipify） ====="
curl 'https://api.ipify.org?format=json'
echo ""

echo ""
echo "===== 2. 查看完整报文（-v） ====="
echo "注意观察：> 请求原文、< 响应原文、* 旁白"
curl -v 'https://api.ipify.org?format=json' 2>&1 | head -30

echo ""
echo "===== 3. 调用 DeepSeek API（需要替换 API KEY） ====="
echo 'curl https://api.deepseek.com/chat/completions \'
echo '  -H "Content-Type: application/json" \'
echo '  -H "Authorization: Bearer sk-YOUR-API-KEY" \'
echo '  -d '"'"'{"model":"deepseek-chat","messages":[{"role":"user","content":"用一句话解释什么是API"}]}'"'"''

echo ""
echo "===== 4. 测试 HTTP 方法 ====="
# POST 示例
curl -X POST https://httpbin.org/post -H "Content-Type: application/json" -d '{"test": "hello"}'
echo ""

echo ""
echo "===== curl 参数速查 ====="
echo "  -H 'Key: Value'    添加请求头"
echo "  -d '...'           添加请求体（POST 数据）"
echo "  -v                 显示完整报文（verbose）"
echo "  -X POST            指定 HTTP 方法"
echo "  -o output.txt      将响应保存到文件"
