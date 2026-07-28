#!/bin/bash
# 模块 8.3：pm2 管理 Next.js 前端

echo "===== 安装 pm2 ====="
npm install -g pm2

echo ""
echo "===== 进入前端项目目录 ====="
cd /home/ubuntu/blog/frontend

echo ""
echo "===== 构建生产版本 ====="
npm run build

echo ""
echo "===== 用 pm2 启动 Next.js ====="
pm2 start npm --name "blog-frontend" -- start
# pm2 start npm = 用 pm2 管理 npm 命令
# --name "blog-frontend" = 进程名
# -- start = npm run start（Next.js 生产模式）

echo ""
echo "===== 查看状态 ====="
pm2 status

echo ""
echo "===== 查看日志 ====="
echo "  pm2 logs blog-frontend"

echo ""
echo "===== 设置开机自启 ====="
pm2 startup
# pm2 会输出一条命令，复制执行

pm2 save
# 保存当前进程列表——重启后自动恢复

echo ""
echo "===== 日常管理命令 ====="
echo "  pm2 status               查看所有进程"
echo "  pm2 restart blog-frontend  重启"
echo "  pm2 stop blog-frontend     停止"
echo "  pm2 delete blog-frontend   从列表中移除"
echo "  pm2 logs blog-frontend     查看日志"
echo "  pm2 monit                  实时监控面板"
