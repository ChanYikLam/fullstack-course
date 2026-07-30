// TodoApp.jsx — 数据驱动界面综合示例
import { useState } from "react";

export default function TodoApp() {
    const [todos, setTodos] = useState([
        { id: 1, text: "学习 React 组件化", done: true },
        { id: 2, text: "理解数据驱动界面", done: true },
        { id: 3, text: "练习 useState 和 useEffect", done: false },
    ]);
    const [input, setInput] = useState("");
    const [filter, setFilter] = useState("all");

    function addTodo() {
        const text = input.trim();
        if (!text) return;

        const newTodo = {
            id: Date.now(),
            text,
            done: false,
        };

        setTodos([...todos, newTodo]); // 不可变更新
        setInput("");
    }

    function toggleTodo(id) {
        setTodos(
            todos.map((todo) =>
                todo.id === id ? { ...todo, done: !todo.done } : todo
            )
        );
    }

    function deleteTodo(id) {
        setTodos(todos.filter((todo) => todo.id !== id));
    }

    // 筛选逻辑
    const filteredTodos = todos.filter((todo) => {
        if (filter === "active") return !todo.done;
        if (filter === "done") return todo.done;
        return true; // "all"
    });

    const remaining = todos.filter((t) => !t.done).length;

    return (
        <div style={{ maxWidth: 500, margin: "0 auto", padding: 20 }}>
            <h2>Todo 应用</h2>
            <p>剩余 {remaining} 项未完成</p>

            {/* 输入区域 */}
            <div style={{ display: "flex", gap: 8, marginBottom: 16 }}>
                <input
                    value={input}
                    onChange={(e) => setInput(e.target.value)}
                    onKeyDown={(e) => e.key === "Enter" && addTodo()}
                    placeholder="输入待办事项..."
                    style={{ flex: 1, padding: "8px 12px", borderRadius: 6, border: "1px solid #ddd" }}
                />
                <button
                    onClick={addTodo}
                    style={{
                        padding: "8px 20px",
                        background: "#0071e3",
                        color: "#fff",
                        border: "none",
                        borderRadius: 6,
                        cursor: "pointer",
                    }}
                >
                    添加
                </button>
            </div>

            {/* 筛选按钮 */}
            <div style={{ display: "flex", gap: 8, marginBottom: 16 }}>
                {["all", "active", "done"].map((f) => (
                    <button
                        key={f}
                        onClick={() => setFilter(f)}
                        style={{
                            padding: "4px 12px",
                            background: filter === f ? "#0071e3" : "#e8e8ed",
                            color: filter === f ? "#fff" : "#333",
                            border: "none",
                            borderRadius: 4,
                            cursor: "pointer",
                            fontSize: "0.85rem",
                        }}
                    >
                        {f === "all" ? "全部" : f === "active" ? "未完成" : "已完成"}
                    </button>
                ))}
            </div>

            {/* 列表 */}
            <ul style={{ listStyle: "none", padding: 0 }}>
                {filteredTodos.map((todo) => (
                    <li
                        key={todo.id}
                        style={{
                            display: "flex",
                            alignItems: "center",
                            gap: 10,
                            padding: "10px 0",
                            borderBottom: "1px solid #eee",
                        }}
                    >
                        <input
                            type="checkbox"
                            checked={todo.done}
                            onChange={() => toggleTodo(todo.id)}
                        />
                        <span
                            style={{
                                flex: 1,
                                textDecoration: todo.done ? "line-through" : "none",
                                color: todo.done ? "#999" : "#333",
                            }}
                        >
                            {todo.text}
                        </span>
                        <button
                            onClick={() => deleteTodo(todo.id)}
                            style={{
                                background: "none",
                                border: "none",
                                color: "#ff3b30",
                                cursor: "pointer",
                                fontSize: "1.2rem",
                            }}
                        >
                            ×
                        </button>
                    </li>
                ))}
                {filteredTodos.length === 0 && (
                    <li style={{ textAlign: "center", color: "#999", padding: 20 }}>
                        暂无数据
                    </li>
                )}
            </ul>
        </div>
    );
}
