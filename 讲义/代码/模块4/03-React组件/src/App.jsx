// App.jsx — 组合所有组件
import Nav from "./components/Nav.jsx";
import Card from "./components/Card.jsx";
import { Clock } from "./components/Clock.jsx";

export default function App() {
    return (
        <div className="app">
            <Nav />

            <main className="main">
                <h1>React 组件演示</h1>
                <p className="subtitle">组件嵌套 + Props + useState 综合示例</p>

                <div className="card-grid">
                    <Card
                        title="组件化开发"
                        description="把数据、结构、样式、行为封装成一个独立的 UI 单元。组件可以像积木一样组合。"
                    />
                    <Card
                        title="数据驱动界面"
                        description="React 最大变化：把操作页面变成改变数据。数据变了，界面自动更新。"
                    />
                    <Card
                        title="生态的力量"
                        description="全世界开发者写好的组件和库，一行 npm install 就能用。不需要从零造轮子。"
                    />
                </div>

                <Clock />
            </main>

            <footer className="footer">
                <p>© 2026 React Demo — 从零到全栈</p>
            </footer>
        </div>
    );
}
