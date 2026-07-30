// Vite Demo — 入口文件
import { greet } from "./utils.js";
import "./style.css";

document.getElementById("app").innerHTML = `
    <div class="container">
        <h1>${greet("Vite 世界")}</h1>
        <p>热更新已生效——修改这个文件，页面会自动刷新！</p>
        <p class="hint">试试修改 src/main.js 或 src/style.css</p>
    </div>
`;
