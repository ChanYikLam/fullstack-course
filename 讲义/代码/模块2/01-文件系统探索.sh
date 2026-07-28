#!/bin/bash
# 模块 2.1：认识你的电脑 — 课堂演示代码
# 在终端中逐行执行，观察每个命令的输出

echo "===== 第一组：你在哪里？周围有什么？ ====="
pwd
echo "上面这行是你当前所在的目录"

echo ""
echo "===== 列出当前目录文件 ====="
ls

echo ""
echo "===== 列出所有文件（含隐藏文件） ====="
ls -la

echo ""
echo "===== 第二组：到处走走 ====="
echo "回到主目录："
cd ~
pwd

echo ""
echo "进入根目录："
cd /
ls

echo ""
echo "进入桌面："
cd ~/Desktop
pwd
ls

echo ""
echo "回到上一个目录："
cd -
pwd

echo ""
echo "===== 第三组：深入理解路径 ====="
echo "根目录结构一览："
cd / && ls

echo ""
echo "你的主目录在哪？"
echo $HOME

echo ""
echo "回到起点："
cd ~
pwd
