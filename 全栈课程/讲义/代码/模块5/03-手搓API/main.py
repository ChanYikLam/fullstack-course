"""
模块 5.3：理解 HTTP，手搓一个 API

零依赖——只使用 Python 标准库 http.server
运行：python3 main.py
测试：curl http://localhost:8000/api/profile
"""

from http.server import BaseHTTPRequestHandler, HTTPServer
import json

# ===== 要返回的数据 =====
profile = {
    "heroTitle": "关于我",
    "heroSubtitle": "项目，创意，灵感，心得，我的作品",
}

# ===== 请求处理器 =====
class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        """处理所有 GET 请求"""

        if self.path == "/api/profile":
            # ─── 1. 状态行 ───（必写！缺了不是合法的 HTTP 响应）
            self.send_response(200)

            # ─── 2. 响应头 ───
            self.send_header("Content-Type", "application/json")
            self.send_header("Access-Control-Allow-Origin", "*")

            # ─── 3. 空行 ───（必写！头和体的分界线）
            self.end_headers()

            # ─── 4. 响应体 ───
            body = json.dumps(profile, ensure_ascii=False)  # 中文原样输出
            self.wfile.write(body.encode("utf-8"))

        elif self.path == "/api/hello":
            self.send_response(200)
            # 试试把 text/html 改成 text/plain，看效果
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.end_headers()
            self.wfile.write("<h1>你好，HTTP！</h1>".encode("utf-8"))

        elif self.path.startswith("/api/echo"):
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            body = json.dumps(
                {"message": "服务器收到你的请求了！", "path": self.path},
                ensure_ascii=False,
            )
            self.wfile.write(body.encode("utf-8"))

        else:
            # 404：资源未找到（必写 end_headers！）
            self.send_response(404)
            self.end_headers()

    def log_message(self, format, *args):
        """自定义日志格式"""
        print(f"[访问] {self.client_address[0]} - {format % args}")


# ===== 启动 =====
PORT = 8000
print(f"后端已启动：http://localhost:{PORT}/api/profile")
print(f"测试命令：curl http://localhost:{PORT}/api/profile")
print("按 Ctrl+C 停止")
HTTPServer(("", PORT), Handler).serve_forever()
