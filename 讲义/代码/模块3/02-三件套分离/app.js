// 模块 3.2：三件套分离 — 脚本文件

document.getElementById("add-project").addEventListener("click", function () {
    const list = document.getElementById("project-list");
    const newItem = document.createElement("li");
    const count = list.children.length + 1;
    newItem.textContent = "新项目 " + count;
    list.appendChild(newItem);

    // 添加一个小动画效果
    newItem.style.opacity = "0";
    newItem.style.transform = "translateY(-10px)";
    newItem.style.transition = "all 0.3s ease";

    requestAnimationFrame(() => {
        newItem.style.opacity = "1";
        newItem.style.transform = "translateY(0)";
    });
});

// 页面加载完成
console.log("页面已就绪！");
