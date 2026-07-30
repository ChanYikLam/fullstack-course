#!/bin/bash
# 模块 3.4：GitHub 与远程同步 — 操作流程

GITHUB_USER="你的GitHub用户名"
REPO_NAME="my-first-site"

echo "========================================"
echo "  GitHub 远程同步完整流程"
echo "========================================"

echo ""
echo "===== Step 1: 浏览器操作 ====="
echo "1. 打开 https://github.com"
echo "2. 登录你的账号"
echo "3. 点击右上角 + 号 → New repository"
echo "4. Repository name: ${REPO_NAME}"
echo "5. 选择 Public（公开）"
echo "6. 不要勾选 Add a README file"
echo "7. 点击 Create repository"
echo ""

echo "===== Step 2: 创建本地项目 ====="
mkdir ~/${REPO_NAME}
cd ~/${REPO_NAME}

# 创建之前的三件套文件（简化版）
cat > index.html << 'HTML'
<!DOCTYPE html>
<html lang="zh-CN">
<head><meta charset="UTF-8"><title>我的网站</title><link rel="stylesheet" href="style.css"></head>
<body><h1>你好，世界！</h1><script src="app.js"></script></body>
</html>
HTML

cat > style.css << 'CSS'
body { font-family: sans-serif; text-align: center; padding: 40px; }
h1 { color: #333; }
CSS

cat > app.js << 'JS'
console.log("页面已加载！");
JS

echo ""
echo "===== Step 3: 初始化 Git 并提交 ====="
git init
git add .
git commit -m "第一个版本：三件套分离的个人主页"

echo ""
echo "===== Step 4: 关联远程仓库并推送 ====="
git remote add origin "https://github.com/${GITHUB_USER}/${REPO_NAME}.git"
git branch -M main

echo "接下来执行推送（需要 GitHub 认证）："
echo "  git push -u origin main"
echo ""
echo "如果密码不生效，GitHub 现在要求用 Personal Access Token 代替密码："
echo "  GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)"
echo "  勾选 repo 权限 → 生成 → 复制 token → 推送时密码输入 token"

echo ""
echo "===== Step 5: 验证 ====="
echo "刷新 GitHub 页面，应该看到你的文件了！"

echo ""
echo "===== Step 6: 练习克隆（在另一个目录） ====="
echo "  cd ~"
echo "  git clone https://github.com/${GITHUB_USER}/${REPO_NAME}.git test-clone"
echo "  cd test-clone && ls"

echo ""
echo "清理本地练习文件："
echo "  cd ~ && rm -rf ${REPO_NAME} test-clone"
