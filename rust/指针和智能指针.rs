// ============================================================
// 文件名: main.rs
// 功能: Rust 指针与智能指针完全教学
// 对标 C++: 原始指针, unique_ptr, shared_ptr, weak_ptr, mutable
// ============================================================

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

// ============================================================
// 第一部分: 原始指针（*const T / *mut T）—— C++ 的 int*
// 警告: 绝大部分 Rust 代码不需要它！仅用于 FFI 或极底层操作
// ============================================================
fn raw_pointer_demo() {
    println!("\n========== 1. 原始指针（*const / *mut）==========");
    let mut num = 42;

    // 从引用安全地创建原始指针（这本身是安全的）
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("原始指针地址: {:?}, {:?}", r1, r2);

    // ⚠️ 解引用原始指针必须放在 unsafe 块中（类似于 C++ 的 "信任我"）
    unsafe {
        // 读取
        println!("通过原始指针读取值: {}", *r1);
        // 修改
        *r2 = 100;
        println!("通过原始指针修改后: {}", *r2);
    }
    // 总结：Rust 极力推荐你永远用引用（&）代替原始指针
}

// ============================================================
// 第二部分: Box<T> —— 唯一的堆内存所有权（对标 unique_ptr）
// 核心: 一个 Box 拥有一个值，不能拷贝，只能移动。
// ============================================================
fn box_demo() {
    println!("\n========== 2. Box<T> （对标 unique_ptr）==========");

    // 在堆上分配一个 i32（类似 C++ 的 auto p = make_unique<int>(5);）
    let b = Box::new(5);
    println!("Box 中的值: {}", b);
    println!("Box 占用栈上指针大小: {} 字节", std::mem::size_of_val(&b));

    // 移动语义（不能拷贝）
    let b2 = b; // b 被移动到 b2，b 失效
    // println!("{}", b); // ❌ 编译错误：b 被移动

    // ---------- 经典应用: 递归类型（链表） ----------
    // 在 C++ 中，链表节点包含 next 指针（指向自身），Rust 需要 Box 来打破无限大小
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>), // 节点包含数据和下一个节点（用 Box 包裹）
        Nil,                  // 空节点
    }
    use List::{Cons, Nil};

    // 创建链表: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("递归链表: {:?}", list);
}

// ============================================================
// 第三部分: Rc<T> —— 单线程引用计数（对标 shared_ptr，但仅单线程）
// 核心: 多个所有者共享只读数据，计数在堆上。
// 注意: Rc 不是线程安全的（没有实现 Send 和 Sync）。
// ============================================================
fn rc_demo() {
    println!("\n========== 3. Rc<T> （对标 shared_ptr，单线程版）==========");

    // 创建引用计数指针
    let rc_a = Rc::new(String::from("Hello, Rc!"));
    println!("初始强引用计数: {}", Rc::strong_count(&rc_a));

    // 克隆（增加引用计数，不拷贝数据！）
    let rc_b = Rc::clone(&rc_a);
    println!("克隆后计数: {}", Rc::strong_count(&rc_a));
    println!("rc_b 内容: {}", rc_b);

    {
        let rc_c = Rc::clone(&rc_a);
        println!("内部作用域计数: {}", Rc::strong_count(&rc_a));
    } // rc_c 离开作用域，计数减 1

    println!("rc_c 离开后计数: {}", Rc::strong_count(&rc_a));

    // 注意：Rc 只能提供不可变引用（&T）。不能通过 Rc 修改值。
    // rc_a.push_str("!"); // ❌ 编译错误：Rc 的数据不可变
}

// ============================================================
// 第四部分: RefCell<T> —— 内部可变性（运行时借用检查）
// C++ 对标: mutable 成员变量，或运行时检查的读写锁
// 核心: 将“编译时借用规则”推迟到“运行时”检查，违反规则会 panic。
// 注意: RefCell 也不是线程安全的。
// ============================================================
fn refcell_demo() {
    println!("\n========== 4. RefCell<T> （内部可变性）==========");

    // 即使 cell 本身不可变（let 默认不可变），我们依然可以修改内部值
    let cell = RefCell::new(42);
    println!("初始值: {}", cell.borrow()); // 不可变借用

    // 修改值（通过 borrow_mut 获取可变引用）
    *cell.borrow_mut() = 100;
    println!("修改后: {}", cell.borrow());

    // ---------- 运行时借用规则检查（违反会 panic） ----------
    // 规则: 要么同时有多个不可变借用（borrow），要么只有 1 个可变借用（borrow_mut）

    let cell2 = RefCell::new(10);
    let r1 = cell2.borrow(); // 不可变借用1
    let r2 = cell2.borrow(); // 不可变借用2（允许）

    // let r3 = cell2.borrow_mut(); // ❌ 运行时 panic!（因为 r1, r2 还在借用中）
    // 注意：如果在同一作用域内强行解开注释，程序会崩溃并报错 "already mutably borrowed"

    println!("r1 = {}, r2 = {}", r1, r2);
    // r1, r2 在这里释放，之后可以安全获取可变借用
    drop(r1);
    drop(r2);

    let mut r3 = cell2.borrow_mut(); // 现在安全了
    *r3 = 999;
    println!("最终值: {}", cell2.borrow());
}

// ============================================================
// 第五部分: 组合拳 —— Rc<RefCell<T>>（共享可变数据）
// C++ 对标: shared_ptr 指向一个可变对象（但 C++ 默认可变，Rust 需显式组合）
// 这是 Rust 中最常见的内部可变+共享所有权模式！
// ============================================================
fn rc_refcell_demo() {
    println!("\n========== 5. Rc<RefCell<T>> （共享 + 可变）==========");

    // 创建一个共享的、可变的整数
    let shared_data = Rc::new(RefCell::new(0));

    // 创建 3 个所有者，都指向同一份数据
    let owner1 = Rc::clone(&shared_data);
    let owner2 = Rc::clone(&shared_data);
    let owner3 = Rc::clone(&shared_data);

    // owner1 修改数据
    *owner1.borrow_mut() += 10;
    // owner2 修改数据
    *owner2.borrow_mut() += 20;
    // owner3 读取数据
    println!("经过 owner1 和 owner2 修改后，owner3 看到的值: {}", owner3.borrow());

    // 强引用计数此时为 4（原始 + 3个克隆）
    println!("强引用计数: {}", Rc::strong_count(&shared_data));
}

// ============================================================
// 第六部分: 循环引用与 Weak<T>（对标 weak_ptr）
// 场景: 父子节点互相引用，必须用 Weak 打破循环，防止内存泄漏。
// ============================================================
fn weak_demo() {
    println!("\n========== 6. Weak<T> （对标 weak_ptr，打破循环）==========");

    // 定义一个简单的节点：包含值，以及指向下一个节点的弱引用（防止循环）
    #[derive(Debug)]
    struct Node {
        value: i32,
        // 因为 Node 之间可能形成循环，用 Weak 表示“不拥有所有权”的关系
        next: Option<Weak<RefCell<Node>>>,
    }

    // 创建两个节点
    let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
    }));
    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        next: None,
    }));

    // 让 node1 -> node2（强引用）
    node1.borrow_mut().next = Some(Rc::downgrade(&node2)); // Rc::downgrade 生成 Weak

    // 尝试从 node1 通过 Weak 升级访问 node2
    if let Some(weak_ref) = &node1.borrow().next {
        // upgrade() 返回 Option<Rc<T>>，如果数据还在就返回 Some
        if let Some(strong_node2) = weak_ref.upgrade() {
            println!("从 node1 成功访问到 node2 的值: {}", strong_node2.borrow().value);
        } else {
            println!("node2 已经被释放了");
        }
    }

    // 关键特性：当 node2 的强引用计数降为 0 时，Weak 指向的数据会被释放。
    drop(node2); // 手动释放 node2（强引用计数归零）
    // 此时 node1 中的 Weak 指向的 node2 已经不存在了
    if let Some(weak_ref) = &node1.borrow().next {
        if weak_ref.upgrade().is_none() {
            println!("✅ node2 已被释放，Weak 不会阻止释放，循环引用被打破！");
        }
    }
    // 注：实际编码中很少手动 drop，这里仅为演示 Weak 机制
}

// ============================================================
// 第七部分: Arc<T> —— 多线程安全的引用计数（对标多线程 shared_ptr）
// 注意: Arc 和 Rc 的 API 几乎一样，区别仅在于 Arc 可以安全地在线程间传递。
// ============================================================
fn arc_demo() {
    println!("\n========== 7. Arc<T> （多线程 shared_ptr）==========");

    // 创建原子引用计数指针
    let arc_data = Arc::new(42);
    let arc_data_clone = Arc::clone(&arc_data);

    // 启动一个线程，安全地读取数据
    let handle = thread::spawn(move || {
        println!("子线程读取到: {}", arc_data_clone);
    });

    // 主线程也读取
    println!("主线程读取到: {}", arc_data);
    handle.join().unwrap();

    // ⚠️ 注意：Arc 默认也是不可变的，如果想多线程修改，必须配合 Mutex 或 RwLock
    // 例如: Arc<Mutex<i32>> 或 Arc<RwLock<i32>>（类似于 C++ 的 shared_ptr<mutex<int>>）
    use std::sync::Mutex;
    let shared_counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("10 个线程累加后结果: {}", *shared_counter.lock().unwrap());
}

// ============================================================
// 第八部分: 综合实战 —— 构建一个简单的可共享可变树结构
// ============================================================
fn combined_advanced_demo() {
    println!("\n========== 8. 综合实战: 树结构（Rc<RefCell<Node>>）==========");

    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        children: Vec<Rc<RefCell<TreeNode>>>,
    }

    // 创建根节点
    let root = Rc::new(RefCell::new(TreeNode {
        value: 1,
        children: vec![],
    }));

    // 创建子节点
    let child1 = Rc::new(RefCell::new(TreeNode {
        value: 2,
        children: vec![],
    }));
    let child2 = Rc::new(RefCell::new(TreeNode {
        value: 3,
        children: vec![],
    }));

    // 将子节点加入根节点的 children 列表（共享所有权 + 可变）
    root.borrow_mut().children.push(Rc::clone(&child1));
    root.borrow_mut().children.push(Rc::clone(&child2));

    // 通过根节点修改 child1 的值（这就是 Rc<RefCell> 的强大之处）
    if let Some(first_child) = root.borrow().children.first() {
        first_child.borrow_mut().value = 20;
    }

    println!("根节点: {:?}", root.borrow());
    println!("child1 现在值: {}", child1.borrow().value);
    println!("child2 现在值: {}", child2.borrow().value);

    // 打印强引用计数：root + child1 + child2 = 3
    println!("root 的强引用计数: {}", Rc::strong_count(&root));
    println!("child1 的强引用计数: {}", Rc::strong_count(&child1));
}

// ============================================================
// 主函数入口
// ============================================================
fn main() {
    println!("🔥 Rust 指针与智能指针完全指南");
    println!("（强烈建议对照 C++ 的 unique_ptr / shared_ptr / weak_ptr 来理解）\n");

    raw_pointer_demo();
    box_demo();
    rc_demo();
    refcell_demo();
    rc_refcell_demo();
    weak_demo();
    arc_demo();
    combined_advanced_demo();

    println!("\n✅ 所有智能指针演示完成！");
    println!("💡 记住黄金法则:");
    println!("  1. 能用 & 就不用 Box");
    println!("  2. 能用 Rc 就不用 Arc（单线程快）");
    println!("  3. 除非必要，别用 RefCell（把检查移到运行时）");
    println!("  4. 遇到循环引用，立刻想到 Weak");
}