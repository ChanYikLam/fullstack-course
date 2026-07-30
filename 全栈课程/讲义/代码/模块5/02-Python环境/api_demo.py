"""api_demo.py — 用 Python 调用 API

前提：已激活虚拟环境，已安装 requests（pip install requests）
运行：python3 api_demo.py
"""

import requests

# ===== 1. GET 请求 =====
print("=== 查询我的公网 IP ===")
resp = requests.get("https://api.ipify.org?format=json")
data = resp.json()
print(f"你的公网 IP 是：{data['ip']}")

# ===== 2. 查看完整响应信息 =====
print(f"\n--- 完整响应信息 ---")
print(f"状态码：{resp.status_code}")
print(f"响应头：{dict(resp.headers)}")
print(f"响应体：{resp.text}")

# ===== 3. POST 请求示例 =====
print("\n=== POST 请求示例 ===")
post_resp = requests.post(
    "https://httpbin.org/post",
    json={"name": "张三", "message": "你好，来自 Python！"},
)
result = post_resp.json()
print(f"POST 状态码：{post_resp.status_code}")
print(f"服务端收到的数据：{result.get('json', {})}")

# ===== 4. 错误处理示例 =====
print("\n=== 错误处理示例 ===")
try:
    bad_resp = requests.get("https://httpbin.org/status/404")
    bad_resp.raise_for_status()  # 非 2xx 状态码会抛出异常
except requests.exceptions.HTTPError as e:
    print(f"HTTP 错误：{e}")
except requests.exceptions.ConnectionError:
    print("网络连接失败")
except requests.exceptions.Timeout:
    print("请求超时")
