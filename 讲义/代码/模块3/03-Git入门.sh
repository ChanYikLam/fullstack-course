#!/bin/bash
# 模块 3.3：Git 入门到实操 — 课堂演示代码

echo "===== 第一步：初始化仓库 ====="
mkdir ~/git-practice
cd ~/git-practice
git init
echo "Initialized empty Git repository"

echo ""
echo "===== 第二步：查看状态（此时为空） ====="
git status

echo ""
echo "===== 第三步：创建文件并第一次提交 ====="
echo "# 我的 Git 练习" > README.md
echo "查看状态——文件未追踪："
git status

echo ""
echo "添加文件到暂存区："
git add README.md
git status

echo ""
echo "创建存档："
git commit -m "初始化项目，添加 README"

echo ""
echo "查看提交历史："
git log --oneline

echo ""
echo "===== 第四步：修改并再次提交 ====="
echo "" >> README.md
echo "新增一行内容" >> README.md
echo "查看文件改动："
git diff

git add README.md
git commit -m "在 README 中新增一行内容"

echo ""
echo "完整提交历史："
git log --oneline

echo ""
echo "===== 第五步：创建 .gitignore ====="
echo "*.log" > .gitignore
echo ".DS_Store" >> .gitignore
touch test.log  # 这个文件会被忽略

echo "查看状态——test.log 应该不出现："
git status

git add .gitignore
git commit -m "添加 .gitignore，忽略日志和系统文件"

echo ""
echo "===== 完整提交历史 ====="
git log --oneline --graph

echo ""
echo "===== 清理 ====="
cd ~
rm -rf git-practice
echo "练习仓库已清理"
