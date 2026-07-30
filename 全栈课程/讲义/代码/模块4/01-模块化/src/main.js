// main.js — 应用入口
// 演示：默认导入、命名导入、混合导入

import log, { APP_NAME } from "./utils.js";
import { getGreeting } from "./greeting.js";
import { getCount, increment } from "./clicker.js";

log(`应用启动：${APP_NAME}`);

// 显示问候语
document.getElementById("greeting").textContent = getGreeting("朋友");

// 点击计数
document.getElementById("clickMe").addEventListener("click", () => {
    const newCount = increment();
    document.getElementById("count").textContent = newCount;
    log(`按钮被点击，当前计数：${newCount}`);
});

// 验证模块作用域隔离
// console.log(count); // ❌ 报错！count 在 clicker.js 里是不可见的
