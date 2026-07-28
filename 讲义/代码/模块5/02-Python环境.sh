#!/bin/bash
# 模块 5.2：Python 安装和环境设置 — 完整操作流程

echo "===== Step 1: 检查 Python ====="
python3 --version
echo "版本号 >= 3.8 即可"

echo ""
echo "===== Step 2: 创建项目目录 ====="
mkdir -p ~/zero-to-tech/backend
cd ~/zero-to-tech/backend
echo "当前目录: $(pwd)"

echo ""
echo "===== Step 3: 创建虚拟环境 ====="
python3 -m venv .venv
echo "虚拟环境已创建在 .venv/ 目录"

echo ""
echo "===== Step 4: 激活虚拟环境 ====="
echo "请手动执行: source .venv/bin/activate"
echo "(Windows: .venv\\Scripts\\activate)"
echo "激活后终端提示符前会出现 (.venv)"

echo ""
echo "===== Step 5: 安装包 ====="
echo "请手动执行:"
echo "  pip install requests"
echo "  pip install fastapi uvicorn"

echo ""
echo "===== Step 6: 生成依赖清单 ====="
echo "请手动执行:"
echo "  pip freeze > requirements.txt"
echo "  cat requirements.txt"

echo ""
echo "===== Step 7: 退出虚拟环境 ====="
echo "  deactivate"

echo ""
echo "===== 关键命令速查 ====="
echo "  激活:    source .venv/bin/activate"
echo "  安装:    pip install 包名"
echo "  卸载:    pip uninstall 包名"
echo "  查看:    pip list"
echo "  导出:    pip freeze > requirements.txt"
echo "  恢复:    pip install -r requirements.txt"
echo "  退出:    deactivate"
