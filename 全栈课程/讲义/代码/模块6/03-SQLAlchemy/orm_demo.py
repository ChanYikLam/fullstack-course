"""
模块 6.3：SQLAlchemy ORM 入门

安装：pip install sqlalchemy
运行：python3 orm_demo.py
"""

from sqlalchemy import create_engine, Column, Integer, String, Float, DateTime
from sqlalchemy.orm import DeclarativeBase, Session
from datetime import datetime

# ===== 1. 连接数据库 =====
engine = create_engine("sqlite:///orm_demo.db", echo=False)
print("数据库已连接：orm_demo.db (SQLite)")

# ===== 2. 定义基类 =====
class Base(DeclarativeBase):
    pass

# ===== 3. 定义模型（模型 = 表的 Python 表示） =====
class User(Base):
    __tablename__ = "users"

    id = Column(Integer, primary_key=True, autoincrement=True)
    name = Column(String(50), nullable=False)
    email = Column(String(100), unique=True, nullable=False)
    age = Column(Integer)
    created_at = Column(DateTime, default=datetime.utcnow)

    def __repr__(self):
        return f"<User(id={self.id}, name='{self.name}', email='{self.email}')>"


# ===== 4. 创建表 =====
Base.metadata.create_all(engine)
print("表已创建：users")

# ===== 5. CRUD 操作 =====
with Session(engine) as session:
    # --- Create ---
    user1 = User(name="张三", email="zhangsan@example.com", age=25)
    user2 = User(name="李四", email="lisi@example.com", age=30)
    user3 = User(name="王五", email="wangwu@example.com", age=28)
    session.add_all([user1, user2, user3])
    session.commit()
    print(f"\n创建了 3 个用户")

    # --- Read（查全部） ---
    all_users = session.query(User).all()
    print(f"\n全部用户 ({len(all_users)} 人)：")
    for u in all_users:
        print(f"  {u}")

    # --- Read（条件 + 排序） ---
    print(f"\n年龄 > 26 的用户：")
    adults = (
        session.query(User)
        .filter(User.age > 26)
        .order_by(User.age.desc())
        .all()
    )
    for u in adults:
        print(f"  {u.name} - {u.age}岁")

    # --- Read（查单个） ---
    user = session.query(User).filter(User.email == "zhangsan@example.com").first()
    print(f"\n精确查找：{user}")

    # --- Update ---
    user.age = 26
    session.commit()
    print(f"更新后：{user.name} 年龄改为 {user.age}")

    # --- Delete ---
    session.delete(user3)
    session.commit()
    remaining = session.query(User).count()
    print(f"\n删除王五后剩余 {remaining} 人")

    # --- 聚合查询 ---
    from sqlalchemy import func
    avg_age = session.query(func.avg(User.age)).scalar()
    print(f"平均年龄：{avg_age:.1f}岁")

print("\n演示完成——数据库连接已关闭")

# 清理
import os
if os.path.exists("orm_demo.db"):
    os.remove("orm_demo.db")
    print("已删除 orm_demo.db")
