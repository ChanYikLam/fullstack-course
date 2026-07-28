// Nav.jsx — 导航栏组件
export default function Nav() {
    return (
        <nav className="nav">
            <div className="nav-inner">
                <span className="nav-logo">React Demo</span>
                <div className="nav-links">
                    <a href="#">首页</a>
                    <a href="#">组件</a>
                    <a href="#">关于</a>
                </div>
            </div>
        </nav>
    );
}
