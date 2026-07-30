// app/page.jsx — Next.js 首页（App Router）
// 对应 URL: /

export default function HomePage() {
    return (
        <main style={{ padding: 40 }}>
            <h1>欢迎来到我的 Next.js 应用</h1>
            <p>这是首页——对应 URL: /</p>
            <a href="/about">前往关于页 →</a>
        </main>
    );
}
