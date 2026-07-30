// Card.jsx — 卡片组件
// 接收 title 和 description 两个 props
export default function Card({ title, description }) {
    return (
        <div className="card">
            <h2 className="card-title">{title}</h2>
            <p className="card-desc">{description}</p>
        </div>
    );
}
