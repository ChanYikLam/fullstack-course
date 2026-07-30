#!/bin/bash
# 模块 2.4：服务器、SSH 和 Nginx 实战 — 完整操作流程
# 注意：替换 你的公网IP 为实际 IP 地址

SERVER_IP="你的公网IP"
SERVER_USER="ubuntu"

echo "========================================"
echo "  服务器实战完整流程"
echo "========================================"

echo ""
echo "===== Step 1: SSH 登录 ====="
echo "执行命令: ssh ${SERVER_USER}@${SERVER_IP}"
echo "首次连接输入 yes，然后输入密码"
echo ""

# 以下命令在服务器上执行
echo "===== 服务器上执行的命令列表 ====="
cat << 'SERVER_COMMANDS'

# --- 登录后 ---

# Step 2: 确认身份和位置
pwd
whoami

# Step 3: 探索根目录
cd /
ls

# Step 4: 更新软件列表
sudo apt update

# Step 5: 安装 Nginx
sudo apt install nginx -y

# Step 6: 确认 Nginx 运行状态
sudo systemctl status nginx
# 看到 "Active: active (running)" 说明成功

# Step 7: 查看 Nginx 配置
cat /etc/nginx/sites-available/default
# 找到 root /var/www/html;

# Step 8: 找到默认页面
cd /var/www/html
ls
# 应该看到 index.nginx-debian.html

# Step 9: 查看默认页面源码
cat index.nginx-debian.html

# Step 10: 用 Vim 修改
sudo vim index.nginx-debian.html
# 按 i → 修改内容 → Esc → :wq → 回车

# Step 11: 浏览器访问
# 打开 http://你的公网IP
# 应该看到你修改后的内容！

# 退出 SSH
exit

SERVER_COMMANDS

echo ""
echo "========================================"
echo "  关键提醒"
echo "========================================"
echo "1. 云平台安全组需放行：22 端口（SSH）+ 80 端口（HTTP）"
echo "2. Vim 生存口诀：i 编辑 → Esc 退出 → :wq 保存退出"
echo "3. sudo = 以管理员身份执行"
echo "4. 浏览器打不开？先检查安全组 80 端口是否放行"
