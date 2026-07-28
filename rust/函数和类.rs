// =============================================
// 第一部分：函数基础（复习 + 深入）
// 对比 C++: 返回值、表达式与语句、多返回值
// =============================================

// 1. 普通函数（无返回值，对应 C++ 的 void）
fn say_hello(name: &str) {
    // 注意：这里只是调用，没有返回值
    println!("你好，{}！", name);
}

// 2. 有返回值的函数（-> 后面跟类型）
// C++ 写法: int add(int a, int b) { return a + b; }
fn add(a: i32, b: i32) -> i32 {
    a + b // ⚠️ 注意没有分号！这是表达式，相当于 return a + b;
    // 当然你也可以显式写 return a + b; 但 Rust 风格推荐省略 return
}

// 3. 返回多个值（利用元组 Tuple，类似 C++ 的 std::tuple 或结构化绑定）
// C++ 写法: std::tuple<int, int> swap(int x, int y) { return {y, x}; }
fn swap_values(x: i32, y: i32) -> (i32, i32) {
    (y, x) // 返回一个元组
}

// 4. 函数作为参数（高阶函数，C++ 用 std::function 或函数指针）
// C++: void apply(std::function<int(int)> f, int v) { ... }
fn apply_twice(f: fn(i32) -> i32, value: i32) -> i32 {
    f(f(value)) // 把函数 f 应用两次
}

// 辅助函数，用于测试高阶函数
fn square(x: i32) -> i32 {
    x * x
}

// =============================================
// 第二部分：类（Class）替代方案 —— 结构体（Struct） + 方法（Impl）
// 对比 C++ 的 class User { private: string name; int age; public: ... };
// =============================================

// 1. 定义结构体（数据成员）
// 注意：Rust 默认所有字段都是私有的（类似 C++ 的 private），除非加 pub
#[derive(Debug)] // 让结构体可以方便地打印调试信息（类似 C++ 重载 operator<<）
struct User {
    name: String,
    age: u32,
    active: bool,
}

// 2. 定义方法（impl 块）
// 这相当于 C++ 中 class 内部的 public 成员函数
impl User {
    // ---------- 构造函数（Constructor） ----------
    // C++ 中的 User(const std::string& name, int age) 
    // Rust 没有重载，通常约定写一个关联函数（静态方法）叫 new
    // 注意：这里没有 self 参数，所以它是关联函数（类似于 C++ 的 static 成员函数）
    pub fn new(name: String, age: u32) -> Self {
        // Self 是类型 User 的别名
        User {
            name,
            age,
            active: true, // 默认新用户激活
        }
    }

    // 多个构造函数：不同的关联函数名（Rust 不支持重载，只能换名字）
    pub fn new_inactive(name: String, age: u32) -> Self {
        User {
            name,
            age,
            active: false,
        }
    }

    // ---------- 不可变成员方法（对应 C++ 的 const 成员函数） ----------
    // &self 相当于 C++ 的 const User* this 或 const User&
    // 只能读，不能改
    pub fn display(&self) {
        // 注意：因为 self 是只读借用，所以不能修改 self.active
        println!("User [{}] 年龄 {}，活跃状态: {}", self.name, self.age, self.active);
    }

    // ---------- 可变成员方法（对应 C++ 的非 const 成员函数） ----------
    // &mut self 相当于 C++ 的 User* this（可修改）
    pub fn birthday(&mut self) {
        self.age += 1;
        println!("🎂 {} 过生日啦！现在 {} 岁", self.name, self.age);
    }

    // ---------- 获取/设置（Getters/Setters） ----------
    // Rust 风格通常直接访问字段，但为了演示封装，写个 Getter
    pub fn get_name(&self) -> &str {
        &self.name // 返回引用，避免拷贝
    }

    // Setter：修改名字
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    // ---------- 消耗自身的方法（极特殊，C++ 没有直接对应） ----------
    // self（不加 &）表示取得所有权，调用后该实例将失效（被移动或释放）
    // 类比 C++ 的 User(User&& other) 移动构造，但这里是方法调用
    pub fn consume(self) {
        println!("{} 被消费掉了！（释放内存）", self.name);
        // 函数结束时，self 被 drop，内存释放
    }

    // ---------- 静态方法（关联函数，不依赖实例） ----------
    // C++: static void print_version() { cout << "v1.0"; }
    pub fn print_version() {
        println!("User 结构体版本 v1.0");
    }
}

// =============================================
// 第三部分：枚举（Enum）的“类”特性（Rust 的王牌）
// C++ 的 enum 只是整数，但 Rust 的枚举可以携带数据，且可以 impl 方法
// =============================================

// 定义一个枚举来表示不同的 IP 地址类型
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),        // 变体携带 4 个 u8 数据
    V6(String),                  // 变体携带 1 个 String
    Unknown,                     // 变体不携带数据（类似 C++ 传统 enum）
}

impl IpAddr {
    // 枚举也可以有方法！
    pub fn is_v4(&self) -> bool {
        // match 匹配当前枚举变体
        match self {
            IpAddr::V4(_, _, _, _) => true,
            _ => false,
        }
    }

    pub fn display(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => println!("IPv4 地址: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(s) => println!("IPv6 地址: {}", s),
            IpAddr::Unknown => println!("未知 IP 类型"),
        }
    }
}

// =============================================
// 第四部分：结构体方法中的 self、&self、&mut self 速查（对比 C++）
// =============================================

struct Demo {
    value: i32,
}

impl Demo {
    // 1. 不可变借用（读） -> C++ const 成员函数
    fn read(&self) -> i32 {
        self.value
    }

    // 2. 可变借用（读+写） -> C++ 非 const 成员函数
    fn write(&mut self, delta: i32) {
        self.value += delta;
    }

    // 3. 取得所有权（移动） -> C++ 中没有直接对应（除非手动 std::move(*this)）
    fn take(self) -> i32 {
        self.value // 取出值，然后 self 被释放
    }
}

// =============================================
// 主函数入口
// =============================================

fn main() {
    // ---------- 测试函数 ----------
    println!("========== 函数测试 ==========");
    say_hello("C++ 程序员");
    println!("1 + 2 = {}", add(1, 2));
    let (a, b) = swap_values(10, 20);
    println!("交换后: a={}, b={}", a, b);
    
    let squared_twice = apply_twice(square, 3); // 先 3^2=9，再 9^2=81
    println!("3 平方两次 = {}", squared_twice);

    // ---------- 测试结构体（类） ----------
    println!("\n========== 结构体（类）测试 ==========");
    
    // 创建实例（调用关联函数 new）
    let mut user1 = User::new(String::from("张三"), 28);
    let user2 = User::new_inactive(String::from("李四"), 35);

    // 调用不可变方法（自动不可变借用）
    user1.display();
    user2.display();

    // 调用可变方法（修改 age）
    user1.birthday();
    
    // 调用 Getter/Setter
    println!("user1 的名字是: {}", user1.get_name());
    user1.set_name(String::from("张伟"));
    println!("修改后的名字: {}", user1.get_name());

    // 调用静态方法（不需要实例）
    User::print_version();

    // 测试 consume（移动所有权）
    user1.consume();
    // user1.display(); // ❌ 这行解开注释会报错！因为 user1 已被 move

    // ---------- 测试枚举方法 ----------
    println!("\n========== 枚举测试 ==========");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let unknown = IpAddr::Unknown;

    home.display();
    loopback.display();
    unknown.display();
    println!("home 是 IPv4 吗？ {}", home.is_v4());
    println!("loopback 是 IPv4 吗？ {}", loopback.is_v4());

    // ---------- self / &self / &mut self 差异演示 ----------
    println!("\n========== self 三种形态对比 ==========");
    let mut demo = Demo { value: 100 };
    println!("只读 value: {}", demo.read());   // 自动 &self
    demo.write(10);                         // 自动 &mut self
    println!("修改后 value: {}", demo.read());
    
    let taken_value = demo.take();           // 移动走 demo
    println!("取走的值: {}", taken_value);
    // println!("{:?}", demo); // ❌ 编译错误，demo 已失效
}