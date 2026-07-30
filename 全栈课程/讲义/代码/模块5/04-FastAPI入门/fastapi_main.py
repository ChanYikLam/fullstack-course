"""
模块 5.4：FastAPI 入门

从手搓 API 到框架——同样的功能，5 行代码搞定

安装：pip install fastapi uvicorn
运行：uvicorn fastapi_main:app --reload
文档：http://localhost:8000/docs
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Optional
from datetime import datetime

# ===== 创建应用 =====
app = FastAPI(
    title="我的第一个 FastAPI 应用",
    description="从手搓到框架的完整历程",
    version="1.0.0",
)

# ===== 允许跨域 =====
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

# ===== 数据 =====
profile = {
    "heroTitle": "关于我",
    "heroSubtitle": "项目，创意，灵感，心得，我的作品",
}

# ===== GET 接口 =====

@app.get("/api/profile")
def get_profile():
    """获取个人信息"""
    return profile


@app.get("/api/hello")
def hello():
    """返回一个简单的问候"""
    return {"message": "你好，来自 FastAPI！", "time": datetime.now().isoformat()}


@app.get("/api/users/{user_id}")
def get_user(user_id: int):
    """路径参数示例：根据 ID 获取用户"""
    return {"user_id": user_id, "name": f"用户{user_id}"}


@app.get("/api/search")
def search(q: str = "", page: int = 1, size: int = 10):
    """查询参数示例：?q=关键词&page=1"""
    return {
        "query": q,
        "page": page,
        "size": size,
        "results": [f"{q}相关结果{i}" for i in range(1, 4)] if q else [],
    }


# ===== POST 接口 =====

class CreateItem(BaseModel):
    """Pydantic 模型——定义请求体结构，自动校验"""
    name: str
    price: float
    description: Optional[str] = None


@app.post("/api/items")
def create_item(item: CreateItem):
    """创建一个新项目——FastAPI 自动校验 name 和 price"""
    return {
        "message": f"创建成功：{item.name}，价格 {item.price}",
        "item": item.model_dump(),
    }


class CreateMessage(BaseModel):
    name: str
    content: str


@app.post("/api/messages")
def create_message(msg: CreateMessage):
    """接收消息并返回"""
    return {
        "id": 1,
        "name": msg.name,
        "content": msg.content,
        "message": f"你好 {msg.name}，已收到你的消息！",
    }


# ===== 健康检查 =====

@app.get("/api/health")
def health():
    """健康检查接口"""
    return {"status": "ok", "timestamp": datetime.now().isoformat()}
