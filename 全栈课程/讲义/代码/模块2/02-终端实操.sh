#!/bin/bash
# 模块 2.2：终端与 Linux 直觉 — 课堂演示代码

echo "===== 第一组：文件操作 ====="
mkdir ~/practice-folder
cd ~/practice-folder
echo "当前目录：$(pwd)"

touch hello.txt
echo "Hello World" > hello.txt
echo "文件内容："
cat hello.txt

echo "第二行文字" >> hello.txt
echo "追加后的内容："
cat hello.txt

echo ""
echo "===== 第二组：复制和移动 ====="
cp hello.txt copy.txt
echo "复制后："
ls

mv copy.txt renamed.txt
echo "重命名后："
ls

mkdir subfolder
mv renamed.txt subfolder/
echo "移动到子文件夹后："
ls subfolder/

echo ""
echo "===== 第三组：目录操作 ====="
cd ~
mkdir -p test/nested/folder
echo "递归创建目录：test/nested/folder"
ls -R test/

echo ""
echo "===== 清理 ====="
rm -rf practice-folder test
echo "已清理练习文件"

echo ""
echo "===== 第四组：Vim 练习（手动操作） ====="
echo "请手动执行："
echo "  vim ~/test.txt"
echo "  按 i → 输入 '我在用 Vim！'"
echo "  按 Esc → 输入 :wq → 回车"
echo "  cat ~/test.txt"
echo "  rm ~/test.txt"
