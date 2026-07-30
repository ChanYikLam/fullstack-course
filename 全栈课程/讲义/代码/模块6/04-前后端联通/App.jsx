// 模块 6.4：前后端数据联通 — React 前端
// 配合 backend.py 使用
// 使用 Vite + React 创建项目后替换 src/App.jsx

import { useState, useEffect } from "react";

export default function App() {
    const [todos, setTodos] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        fetch("http://localhost:8000/api/todos")
            .then((res) => {
                if (!res.ok) throw new Error(`HTTP ${res.status}`);
                return res.json();
            })
            .then((data) => {
                setTodos(data);
                setLoading(false);
            })
            .catch((err) => {
                setError(err.message);
                setLoading(false);
            });
    }, []);

    if (loading) {
        return (
            <div style={styles.center}>
                <p>加载中...</p>
            </div>
        );
    }

    if (error) {
        return (
            <div style={styles.center}>
                <p style={{ color: "#ff3b30" }}>出错了：{error}</p>
                <p style={{ color: "#888" }}>请确保后端已启动（uvicorn backend:app --reload）</p>
            </div>
        );
    }

    const completed = todos.filter((t) => t.completed).length;

    return (
        <div style={styles.container}>
            <h1 style={styles.title}>我的 Todo 列表</h1>
            <p style={styles.stats}>
                共 {todos.length} 项，已完成 {completed} 项
            </p>

            <ul style={styles.list}>
                {todos.map((todo) => (
                    <li
                        key={todo.id}
                        style={{
                            ...styles.item,
                            textDecoration: todo.completed ? "line-through" : "none",
                            color: todo.completed ? "#999" : "#333",
                        }}
                    >
                        <span
                            style={{
                                ...styles.checkbox,
                                background: todo.completed ? "#34c759" : "#e8e8ed",
                            }}
                        >
                            {todo.completed ? "✓" : ""}
                        </span>
                        {todo.title}
                    </li>
                ))}
            </ul>
        </div>
    );
}

const styles = {
    center: {
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        minHeight: "100vh",
        fontFamily: "system-ui, sans-serif",
    },
    container: {
        maxWidth: 500,
        margin: "40px auto",
        padding: "20px",
        fontFamily: "system-ui, sans-serif",
    },
    title: {
        fontSize: "2rem",
        marginBottom: "8px",
    },
    stats: {
        color: "#888",
        marginBottom: "24px",
    },
    list: {
        listStyle: "none",
        padding: 0,
    },
    item: {
        display: "flex",
        alignItems: "center",
        gap: "12px",
        padding: "12px 0",
        borderBottom: "1px solid #eee",
        fontSize: "1.05rem",
    },
    checkbox: {
        display: "inline-flex",
        alignItems: "center",
        justifyContent: "center",
        width: "24px",
        height: "24px",
        borderRadius: "6px",
        color: "#fff",
        fontSize: "14px",
        flexShrink: 0,
    },
};
