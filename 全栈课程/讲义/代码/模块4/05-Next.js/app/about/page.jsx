// app/about/page.jsx — Next.js 关于页
// 文件夹名 = 路由路径！app/about/page.jsx → /about

export default function AboutPage() {
    return (
        <main style={{ padding: 40 }}>
            <h1>关于我们</h1>
            <p>这是关于页——对应 URL: /about</p>
            <p>Next.js 的文件路由：文件夹结构直接映射为 URL 路径</p>
            <a href="/">← 返回首页</a>
        </main>
    );
}
