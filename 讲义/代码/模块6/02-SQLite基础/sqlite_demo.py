"""
模块 6.2：SQLite 数据库入门

Python 标准库 sqlite3——零依赖
运行：python3 sqlite_demo.py
"""

import sqlite3
from datetime import datetime

# ===== 1. 连接数据库（文件不存在会自动创建） =====
conn = sqlite3.connect("demo.db")
cursor = conn.cursor()
print("数据库已连接：demo.db")

# ===== 2. 创建表 =====
cursor.execute("""
    CREATE TABLE IF NOT EXISTS books (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        author TEXT NOT NULL,
        year INTEGER,
        rating REAL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
""")
print("表已创建：books")

# ===== 3. 插入数据 =====
books_data = [
    ("深入理解计算机系统", "Randal E. Bryant", 2015, 9.7),
    ("JavaScript 高级程序设计", "Matt Frisbie", 2019, 9.0),
    ("Python 编程：从入门到实践", "Eric Matthes", 2022, 9.2),
    ("设计模式", "Erich Gamma", 1994, 9.1),
    ("重构：改善既有代码的设计", "Martin Fowler", 2018, 9.3),
    ("代码整洁之道", "Robert C. Martin", 2008, 8.8),
]

cursor.executemany(
    "INSERT INTO books (title, author, year, rating) VALUES (?, ?, ?, ?)",
    books_data,
)
conn.commit()
print(f"插入了 {cursor.rowcount} 本书")

# ===== 4. 查询 =====

print("\n--- 所有图书（按评分降序） ---")
cursor.execute("SELECT title, author, year, rating FROM books ORDER BY rating DESC")
for row in cursor.fetchall():
    print(f"  《{row[0]}》 {row[1]} ({row[2]}) - {row[3]}分")

print("\n--- 评分 ≥ 9.0 的图书 ---")
cursor.execute(
    "SELECT title, rating FROM books WHERE rating >= ? ORDER BY rating DESC", (9.0,)
)
for row in cursor.fetchall():
    print(f"  《{row[0]}》 - {row[1]}分")

print("\n--- 2020 年后出版的图书 ---")
cursor.execute("SELECT title, year FROM books WHERE year >= 2020 ORDER BY year DESC")
for row in cursor.fetchall():
    print(f"  《{row[0]}》 ({row[1]}年)")

print("\n--- 统计信息 ---")
cursor.execute("SELECT COUNT(*) as total, AVG(rating) as avg_rating FROM books")
total, avg = cursor.fetchone()
print(f"  共 {total} 本书，平均评分 {avg:.2f}")

# ===== 5. 更新 =====
cursor.execute("UPDATE books SET rating = ? WHERE title = ?", (9.5, "Python 编程：从入门到实践"))
conn.commit()
print("\n已更新《Python 编程：从入门到实践》的评分")

# ===== 6. 删除 =====
cursor.execute("DELETE FROM books WHERE year < 2000")
conn.commit()
print(f"已删除 {cursor.rowcount} 本 2000 年前的旧书")

print("\n--- 最终图书列表 ---")
cursor.execute("SELECT title, year, rating FROM books ORDER BY rating DESC")
for row in cursor.fetchall():
    print(f"  《{row[0]}》 ({row[1]}) - {row[2]}分")

conn.close()
print("\n数据库连接已关闭")

# 清理（可选）
import os
if os.path.exists("demo.db"):
    os.remove("demo.db")
    print("已删除 demo.db")
