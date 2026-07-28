"""
模块 6.4：前后端数据联通 — FastAPI 后端

安装：pip install fastapi uvicorn sqlalchemy
运行：uvicorn backend:app --reload
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.orm import DeclarativeBase, Session

app = FastAPI(title="前后端联通 Demo")

# CORS——让前端能调用
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

# 数据库
engine = create_engine("sqlite:///todos.db")


class Base(DeclarativeBase):
    pass


class Todo(Base):
    __tablename__ = "todos"
    id = Column(Integer, primary_key=True, autoincrement=True)
    title = Column(String(200), nullable=False)
    completed = Column(Integer, default=0)


Base.metadata.create_all(engine)

# 种子数据
with Session(engine) as s:
    if s.query(Todo).count() == 0:
        s.add_all(
            [
                Todo(title="学习 React 组件化"),
                Todo(title="理解数据驱动界面"),
                Todo(title="配置 FastAPI 后端"),
                Todo(title="实现前后端数据联通", completed=1),
                Todo(title="学习 HTTP 协议"),
                Todo(title="部署到服务器", completed=1),
            ]
        )
        s.commit()


@app.get("/api/todos")
def get_todos():
    with Session(engine) as s:
        todos = s.query(Todo).all()
        return [
            {"id": t.id, "title": t.title, "completed": bool(t.completed)}
            for t in todos
        ]


@app.get("/api/health")
def health():
    return {"status": "ok", "database": "connected"}
