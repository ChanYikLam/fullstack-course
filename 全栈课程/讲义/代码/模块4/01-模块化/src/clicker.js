// clicker.js — 计数器模块

let count = 0; // 模块内部变量——外部不可见

export function getCount() {
    return count;
}

export function increment() {
    count = count + 1;
    return count;
}
