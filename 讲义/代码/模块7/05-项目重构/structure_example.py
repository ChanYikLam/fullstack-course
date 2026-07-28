"""
模块 6.5：JWT 认证 — 后端重构示例

展示如何将"全写在一个文件"重构为"database / models / schemas / routers"结构
这是模块 7 项目重构的预热
"""

# ===== database.py =====
# 数据库连接配置
from sqlalchemy import create_engine
from sqlalchemy.orm import Session
import os

DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///app.db")
engine = create_engine(DATABASE_URL, echo=False)


def get_db():
    """FastAPI 依赖注入——每个请求获取一个数据库会话"""
    with Session(engine) as session:
        yield session


# ===== models.py =====
# SQLAlchemy 数据模型
from sqlalchemy.orm import DeclarativeBase
from sqlalchemy import Column, Integer, String, DateTime, ForeignKey
from datetime import datetime


class Base(DeclarativeBase):
    pass


class User(Base):
    __tablename__ = "users"
    id = Column(Integer, primary_key=True, autoincrement=True)
    username = Column(String(50), unique=True, nullable=False)
    password_hash = Column(String(200), nullable=False)
    created_at = Column(DateTime, default=datetime.utcnow)


class Post(Base):
    __tablename__ = "posts"
    id = Column(Integer, primary_key=True, autoincrement=True)
    title = Column(String(200), nullable=False)
    content = Column(String(5000), nullable=False)
    user_id = Column(Integer, ForeignKey("users.id"), nullable=False)
    created_at = Column(DateTime, default=datetime.utcnow)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow)


# ===== schemas.py =====
# Pydantic 请求/响应模型
from pydantic import BaseModel
from typing import Optional
from datetime import datetime


class UserOut(BaseModel):
    id: int
    username: str
    created_at: datetime

    class Config:
        from_attributes = True


class PostCreate(BaseModel):
    title: str
    content: str


class PostOut(BaseModel):
    id: int
    title: str
    content: str
    user_id: int
    created_at: datetime
    updated_at: datetime

    class Config:
        from_attributes = True


class PaginatedResponse(BaseModel):
    items: list
    total: int
    page: int
    size: int
    pages: int


# ===== routers/posts.py =====
# 文章相关路由
from fastapi import APIRouter, Depends, HTTPException, Query
from sqlalchemy.orm import Session

router = APIRouter(prefix="/api/posts", tags=["posts"])


@router.get("/", response_model=PaginatedResponse)
def get_posts(
    page: int = Query(1, ge=1),
    size: int = Query(10, ge=1, le=100),
    db: Session = Depends(get_db),
):
    """获取文章列表（分页）"""
    total = db.query(Post).count()
    posts = (
        db.query(Post)
        .order_by(Post.created_at.desc())
        .offset((page - 1) * size)
        .limit(size)
        .all()
    )
    return {
        "items": posts,
        "total": total,
        "page": page,
        "size": size,
        "pages": (total + size - 1) // size,
    }


# ===== main.py =====
# FastAPI 入口——简洁！
from fastapi import FastAPI

app = FastAPI()
# 注册路由
# app.include_router(posts.router)
# app.include_router(auth.router)

# 启动
# uvicorn main:app --reload
