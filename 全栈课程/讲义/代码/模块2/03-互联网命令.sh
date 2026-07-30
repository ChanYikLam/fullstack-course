#!/bin/bash
# 模块 2.3：互联网是怎么工作的 — 课堂演示代码

echo "===== 查看本机 IP 地址 ====="
echo "=== macOS / Linux ==="
ifconfig | grep "inet " 2>/dev/null || ip addr show | grep "inet " 2>/dev/null

echo ""
echo "===== DNS 解析：域名 → IP ====="
nslookup bilibili.com

echo ""
echo "===== 追踪路由路径（可能很慢或被墙） ====="
echo "正在追踪到 bilibili.com 的路由..."
traceroute bilibili.com 2>/dev/null || traceroute -m 10 bilibili.com 2>/dev/null || echo "traceroute 不可用，跳过"

echo ""
echo "===== 测试延迟 ====="
ping -c 5 bilibili.com

echo ""
echo "===== 查看本机监听端口 ====="
echo "=== macOS ==="
lsof -iTCP -sTCP:LISTEN -P 2>/dev/null || echo "需要 sudo 权限"
echo ""
echo "=== Linux 请使用 ==="
echo "  ss -tlnp"
