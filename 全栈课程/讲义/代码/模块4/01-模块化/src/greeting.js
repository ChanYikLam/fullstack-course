// greeting.js — 问候模块

export function getGreeting(name) {
    const hour = new Date().getHours();
    if (hour < 12) return `早上好，${name}！`;
    if (hour < 18) return `下午好，${name}！`;
    return `晚上好，${name}！`;
}
