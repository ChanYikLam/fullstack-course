// Clock.jsx — 实时时钟组件（useEffect 示例）
import { useState, useEffect } from "react";

export function Clock() {
    const [time, setTime] = useState(new Date());

    useEffect(() => {
        console.log("时钟启动");
        const timer = setInterval(() => {
            setTime(new Date());
        }, 1000);

        // 清理函数——组件卸载时执行
        return () => {
            console.log("时钟停止");
            clearInterval(timer);
        };
    }, []); // 空数组 = 只在挂载/卸载时执行

    return (
        <div className="clock">
            <span className="clock-label">当前时间：</span>
            <span className="clock-time">
                {time.toLocaleTimeString("zh-CN", {
                    hour: "2-digit",
                    minute: "2-digit",
                    second: "2-digit",
                })}
            </span>
        </div>
    );
}
