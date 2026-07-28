// utils.js — 工具函数模块（命名导出 + 默认导出）

// 命名导出
export function formatDate(date) {
    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, "0");
    const d = String(date.getDate()).padStart(2, "0");
    return `${y}-${m}-${d}`;
}

export const APP_NAME = "我的模块化应用";

// 默认导出
export default function log(message) {
    console.log(`[${formatDate(new Date())}] ${message}`);
}
