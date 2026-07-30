#!/bin/bash
# 模块 3.5：服务器部署与 Nginx 配置 — 操作流程
# 在服务器上执行

SERVER_IP="你的公网IP"
GITHUB_USER="你的GitHub用户名"
REPO_NAME="my-first-site"

echo "========================================"
echo "  服务器部署完整流程"
echo "========================================"

cat << 'SERVER_SCRIPT'
# ===== Step 1: SSH 登录 =====
# 在你的电脑终端执行：
# ssh ubuntu@你的公网IP

# ===== Step 2: 安装 Git =====
sudo apt install git -y

# ===== Step 3: 进入 /var/www =====
cd /var/www

# 如果权限不足：
sudo chown -R $USER:$USER /var/www

# ===== Step 4: 克隆代码 =====
git clone https://github.com/你的GitHub用户名/my-first-site.git
cd my-first-site
ls
# 确认 index.html、style.css、app.js 都在

# ===== Step 5: 修改 Nginx 配置 =====
sudo vim /etc/nginx/sites-available/default

# 找到 root 那一行（大约在第 40-50 行），改成：
# root /var/www/my-first-site;

# ===== Step 6: 检查配置 =====
sudo nginx -t
# 应该看到: syntax is ok / test is successful

# ===== Step 7: 重启 Nginx =====
sudo systemctl restart nginx

# ===== Step 8: 验证 =====
# 浏览器访问 http://你的公网IP
# 应该看到你的个人主页！

# ===== 后续更新流程 =====
# 本地改了代码后：
#   git add . && git commit -m "更新内容" && git push
# 服务器上更新：
#   cd /var/www/my-first-site && git pull

SERVER_SCRIPT

echo ""
echo "========================================"
echo "  完整链路回顾"
echo "========================================"
echo "本地编写代码 → git push → GitHub 云端"
echo "                                  ↓"
echo "                           服务器 git pull"
echo "                                  ↓"
echo "                           Nginx 提供访问"
echo "                                  ↓"
echo "                         公网可访问！"
