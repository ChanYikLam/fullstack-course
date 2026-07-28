"""
模块 6.5：JWT 用户认证完整实现

安装：pip install fastapi uvicorn sqlalchemy python-jose[cryptography] passlib[bcrypt]
运行：uvicorn auth_demo:app --reload
"""

from fastapi import FastAPI, Depends, HTTPException
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.orm import DeclarativeBase, Session
from pydantic import BaseModel
from passlib.context import CryptContext
from jose import JWTError, jwt
from datetime import datetime, timedelta

# ===== 配置 =====
SECRET_KEY = "dev-secret-key-change-in-production-use-env-var"
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 60

app = FastAPI(title="JWT 认证 Demo")
security = HTTPBearer()
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

app.add_middleware(CORSMiddleware, allow_origins=["*"], allow_methods=["*"], allow_headers=["*"])

# ===== 数据库 =====
engine = create_engine("sqlite:///auth_demo.db")


class Base(DeclarativeBase):
    pass


class User(Base):
    __tablename__ = "users"
    id = Column(Integer, primary_key=True, autoincrement=True)
    username = Column(String(50), unique=True, nullable=False)
    password_hash = Column(String(200), nullable=False)
    created_at = Column(String(30), default=lambda: datetime.utcnow().isoformat())


Base.metadata.create_all(engine)

# ===== 请求/响应模型 =====
class RegisterRequest(BaseModel):
    username: str
    password: str


class LoginRequest(BaseModel):
    username: str
    password: str


# ===== 工具函数 =====
def create_access_token(user_id: int) -> str:
    expire = datetime.utcnow() + timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    payload = {"sub": str(user_id), "exp": expire, "iat": datetime.utcnow()}
    return jwt.encode(payload, SECRET_KEY, algorithm=ALGORITHM)


def get_current_user(credentials: HTTPAuthorizationCredentials = Depends(security)):
    """验证 JWT 并返回当前用户"""
    try:
        payload = jwt.decode(credentials.credentials, SECRET_KEY, algorithms=[ALGORITHM])
        user_id = int(payload.get("sub"))
    except JWTError:
        raise HTTPException(status_code=401, detail="Token 无效或已过期")

    with Session(engine) as s:
        user = s.query(User).filter(User.id == user_id).first()
        if not user:
            raise HTTPException(status_code=401, detail="用户不存在")
        return user


# ===== 接口 =====
@app.post("/api/auth/register")
def register(req: RegisterRequest):
    with Session(engine) as s:
        if s.query(User).filter(User.username == req.username).first():
            raise HTTPException(status_code=400, detail="用户名已存在")

        user = User(username=req.username, password_hash=pwd_context.hash(req.password))
        s.add(user)
        s.commit()
        return {"message": "注册成功", "username": req.username}


@app.post("/api/auth/login")
def login(req: LoginRequest):
    with Session(engine) as s:
        user = s.query(User).filter(User.username == req.username).first()
        if not user or not pwd_context.verify(req.password, user.password_hash):
            raise HTTPException(status_code=401, detail="用户名或密码错误")

        token = create_access_token(user.id)
        return {"access_token": token, "token_type": "bearer"}


@app.get("/api/auth/me")
def get_me(user: User = Depends(get_current_user)):
    return {"id": user.id, "username": user.username, "created_at": user.created_at}


@app.get("/api/protected")
def protected_endpoint(user: User = Depends(get_current_user)):
    return {
        "message": "这是一个受保护的接口",
        "user": user.username,
        "secret_data": "只有登录用户能看到的数据",
    }


@app.get("/api/public")
def public_endpoint():
    return {"message": "这是一个公开接口", "secret_data": "任何人都能看到"}
