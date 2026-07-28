// ============================================================
// 文件名: main.rs
// 功能: Rust 特征（Trait）完整教学 —— 对标 C++ 的虚函数与模板
// 涵盖: 定义、实现、默认方法、静态派发、动态派发、关联类型、常用标准 Trait
// ============================================================

use std::fmt; // 用于自定义 Display trait

// ============================================================
// 第一部分: 定义与实现 Trait（接口）
// C++ 对标: class Animal { public: virtual void speak() const = 0; };
// ============================================================

// 定义一个 Trait（特征）
trait Animal {
    // 必须实现的方法（类似 C++ 纯虚函数）
    fn speak(&self); // 注意：&self 相当于 C++ 的 const 成员函数

    // 带返回值的方法
    fn name(&self) -> &str;

    // ---- 默认方法（类似 C++ 的虚函数有默认实现） ----
    // 实现该 Trait 的类型可以选择重写，也可以直接用这个默认实现
    fn greet(&self) {
        println!("你好，我是一只动物。");
    }
}

// 定义结构体 Dog（C++ 类比: class Dog : public Animal）
struct Dog {
    name: String,
    breed: String,
}

// 为 Dog 实现 Animal Trait（C++ 类比: void Dog::speak() const override { ... }）
impl Animal for Dog {
    fn speak(&self) {
        println!("{} 汪汪叫！", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }

    // greet 使用默认实现，不重写
}

// 定义结构体 Cat
struct Cat {
    name: String,
    fur_color: String,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} 喵喵叫！", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }

    // 重写默认方法（自定义问候语）
    fn greet(&self) {
        println!("{} 优雅地蹭了蹭你的腿。", self.name);
    }
}

// ============================================================
// 第二部分: Trait 作为参数 —— 静态派发（Static Dispatch）
// C++ 对标: template <typename T> void make_speak(const T& animal)
// 优势: 零运行时开销，编译器为每个类型生成专属代码（单态化）
// ============================================================

// 方式 1: 使用 impl Trait 语法（推荐，简洁）
fn make_speak_impl(animal: &impl Animal) {
    animal.speak();
}

// 方式 2: 使用泛型 + Trait 边界（Trait Bound），更灵活（比如可加多个 Trait）
fn make_speak_generic<T: Animal>(animal: &T) {
    animal.speak();
}

// 方式 3: 使用 where 子句（当约束较多时，可读性更好）
fn make_speak_where<T>(animal: &T)
where
    T: Animal,
{
    animal.speak();
}

// ---- 多个 Trait 约束示例 ----
// 假设还有一个 Sleep trait
trait Sleep {
    fn sleep(&self);
}

impl Sleep for Dog {
    fn sleep(&self) {
        println!("{} 打呼噜了...", self.name);
    }
}

// 同时要求 Animal + Sleep
fn take_nap<T: Animal + Sleep>(animal: &T) {
    animal.greet();
    animal.sleep();
}

// ============================================================
// 第三部分: Trait 作为参数 —— 动态派发（Dynamic Dispatch）
// C++ 对标: void make_speak(Animal* animal) { animal->speak(); }
// 使用 dyn Trait + 引用/指针（Box/Rc），有虚表（vtable）开销
// 适用场景: 运行时才知道具体类型，比如异构集合（Vec<Box<dyn Animal>>）
// ============================================================

fn make_speak_dynamic(animal: &dyn Animal) {
    animal.speak();
}

// 演示动态派发的集合（C++ 类比: std::vector<Animal*>）
fn dynamic_collection_demo() {
    println!("\n--- 动态派发集合演示 ---");
    let dog = Dog {
        name: String::from("旺财"),
        breed: String::from("金毛"),
    };
    let cat = Cat {
        name: String::from("咪咪"),
        fur_color: String::from("橘色"),
    };

    // 使用 Box<dyn Animal> 把不同类型装箱统一类型
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];

    for animal in animals {
        // 运行时动态调用虚表中的 speak 函数
        animal.speak();
        // 注意: 如果使用动态派发，无法调用 Animal 的默认方法 greet？ 可以，因为 greet 在 Trait 中定义。
        animal.greet();
    }
}

// ============================================================
// 第四部分: 关联类型（Associated Types）
// C++ 对标: 模板中的 typename（例如 template<typename Item> class Container）
// 但是 Rust 的关联类型将类型占位符放在 Trait 内部，实现时指定
// ============================================================

// 定义一个容器 Trait，带关联类型 Item
trait Container {
    // 占位类型，实现时需要指定具体类型
    type Item;

    // 添加元素
    fn add(&mut self, item: Self::Item);

    // 获取第一个元素（引用）
    fn first(&self) -> Option<&Self::Item>;

    // 获取长度
    fn len(&self) -> usize;
}

// 实现一个简单的包装容器（包装 Vec）
struct MyContainer<T> {
    items: Vec<T>,
}

impl<T> MyContainer<T> {
    fn new() -> Self {
        MyContainer { items: Vec::new() }
    }
}

// 为 MyContainer 实现 Container Trait
impl<T> Container for MyContainer<T> {
    // 关联类型 Item 被指定为 T
    type Item = T;

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn first(&self) -> Option<&T> {
        self.items.first()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// ============================================================
// 第五部分: 标准库常用 Trait（Debug, Clone, Display）
// C++ 对标: operator<<, 拷贝构造函数
// ============================================================

// 1. #[derive(Debug)] —— 自动实现 Debug Trait，用于格式化打印（{:?}）
//    C++ 对标: 手动写的 operator<<(std::ostream&, const MyClass&)
#[derive(Debug, Clone)] // Clone 对标 C++ 的拷贝构造函数
struct Point {
    x: i32,
    y: i32,
}

// 2. 手动实现 Display Trait（用于 {} 友好输出），类似 C++ 的 operator<<
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 3. Drop Trait（析构函数），虽然很少手动实现，但了解其存在
impl Drop for Point {
    fn drop(&mut self) {
        // 这里会在 Point 离开作用域时自动调用
        // 类似 C++ 的 ~Point()
        // 注意：Rust 会自动处理内存释放，一般不用自己写，这里仅演示
        println!("[Drop] 坐标 ({}, {}) 被释放了", self.x, self.y);
    }
}

// ============================================================
// 第六部分: Trait 继承（Trait Bound on Trait）与 孤儿规则（Orphan Rule）说明
// C++ 对标: 子类继承父类接口
// ============================================================

// 定义一个更具体的 Trait: FlyingAnimal，要求必须先实现 Animal
trait FlyingAnimal: Animal {
    fn fly(&self);
}

// 定义 Bird
struct Bird {
    name: String,
}

impl Animal for Bird {
    fn speak(&self) {
        println!("{} 叽叽喳喳！", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// Bird 实现 FlyingAnimal（因为已经实现了 Animal）
impl FlyingAnimal for Bird {
    fn fly(&self) {
        println!("{} 在天空中飞翔！", self.name);
    }
}

// ---- 孤儿规则（Orphan Rule）说明 ----
// 规则: 对于 impl Trait for Type，Trait 或 Type 至少有一个必须在当前 crate 中定义。
// 即: 你不能为外部类型实现外部 Trait（例如 不能为自己的 Trait 实现 Vec<T>，除非本地 Trait）
// 这类似于 C++ 中你不能在命名空间外为 std::vector 插入 operator<< 吗？（其实可以，但 Rust 严格禁止以避免冲突）。
// 解决办法: 使用 Newtype 模式（包装成元组结构体）。

// ============================================================
// 第七部分: 综合实战 —— 使用 Trait 实现多态计算
// ============================================================

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// 接受任意实现 Shape 的引用，静态派发（零开销）
fn print_shape_info(shape: &impl Shape) {
    println!("面积: {:.2}, 周长: {:.2}", shape.area(), shape.perimeter());
}

// ============================================================
// 主函数入口
// ============================================================

fn main() {
    println!("========== 第一部分: 定义与实现 Trait ==========");
    let dog = Dog {
        name: String::from("旺财"),
        breed: String::from("金毛"),
    };
    let cat = Cat {
        name: String::from("咪咪"),
        fur_color: String::from("橘色"),
    };

    dog.speak();
    dog.greet();
    cat.speak();
    cat.greet();

    println!("\n========== 第二部分: 静态派发（泛型） ==========");
    // 三种调用方式效果一样，编译时完全展开
    make_speak_impl(&dog);
    make_speak_generic(&cat);
    make_speak_where(&dog);

    // 多个 Trait 约束
    take_nap(&dog);
    // take_nap(&cat); // ❌ 编译错误：Cat 没有实现 Sleep

    println!("\n========== 第三部分: 动态派发（dyn） ==========");
    let dog_dyn = Dog {
        name: String::from("小黑"),
        breed: String::from("拉布拉多"),
    };
    let cat_dyn = Cat {
        name: String::from("花花"),
        fur_color: String::from("黑白"),
    };
    make_speak_dynamic(&dog_dyn);
    make_speak_dynamic(&cat_dyn);
    dynamic_collection_demo();

    println!("\n========== 第四部分: 关联类型 ==========");
    let mut container = MyContainer::new();
    container.add(10);
    container.add(20);
    container.add(30);
    println!("容器长度: {}", container.len());
    if let Some(first) = container.first() {
        println!("第一个元素: {}", first);
    }

    println!("\n========== 第五部分: 标准 Trait（Debug, Display, Drop） ==========");
    let p = Point { x: 5, y: 10 };
    // Debug 输出（派生）
    println!("Debug 输出: {:?}", p);
    // Display 输出（手动实现）
    println!("Display 输出: {}", p);
    {
        let p2 = p.clone(); // 克隆（因为派生 Clone）
        println!("克隆的 p2: {}", p2);
    } // p2 离开作用域，触发 Drop（打印释放信息）
    // p 在这里仍然有效

    println!("\n========== 第六部分: Trait 继承（FlyingAnimal） ==========");
    let bird = Bird {
        name: String::from("小翠"),
    };
    bird.speak();
    bird.fly();

    println!("\n========== 第七部分: 实战 —— 形状计算 ==========");
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    print_shape_info(&circle);
    print_shape_info(&rect);

    // 动态派发集合
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle), Box::new(rect)];
    for shape in &shapes {
        println!("面积: {:.2}", shape.area());
    }

    println!("\n✅ 所有 Trait 演示完成！");
    // p 在这里离开作用域，自动 drop（会在 main 结束时打印释放信息）
}