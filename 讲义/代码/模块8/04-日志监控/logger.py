"""
模块 8.4：Python logging 配置

使用方法：
  from logger import setup_logger
  logger = setup_logger("myapp")
  logger.info("应用启动")
"""

import logging
import sys
import os
from logging.handlers import RotatingFileHandler


def setup_logger(name: str = "app", log_dir: str = "logs") -> logging.Logger:
    """
    配置日志——同时输出到文件和终端
    - 文件：自动轮转（10MB / 文件，保留 5 个旧文件）
    - 终端：实时查看
    """
    logger = logging.getLogger(name)
    logger.setLevel(logging.INFO)

    # 如果已经有 handler 了，跳过（防止重复添加）
    if logger.handlers:
        return logger

    # 日志格式
    fmt = logging.Formatter(
        "%(asctime)s [%(levelname)-7s] %(name)s:%(lineno)d - %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )

    # 输出到文件（带轮转）
    os.makedirs(log_dir, exist_ok=True)
    file_handler = RotatingFileHandler(
        os.path.join(log_dir, f"{name}.log"),
        maxBytes=10 * 1024 * 1024,  # 10MB
        backupCount=5,  # 保留 5 个旧文件
        encoding="utf-8",
    )
    file_handler.setLevel(logging.INFO)
    file_handler.setFormatter(fmt)
    logger.addHandler(file_handler)

    # 输出到终端
    console_handler = logging.StreamHandler(sys.stdout)
    console_handler.setLevel(logging.INFO)
    console_handler.setFormatter(fmt)
    logger.addHandler(console_handler)

    return logger


# ===== 使用示例 =====
if __name__ == "__main__":
    logger = setup_logger("demo")

    logger.debug("这是一条调试信息")  # 不会显示（级别是 INFO）
    logger.info("应用启动完成")
    logger.warning("磁盘空间不足 80%")
    logger.error("数据库连接失败", exc_info=False)

    try:
        1 / 0
    except Exception:
        logger.exception("捕获到异常")  # 自动附带堆栈
