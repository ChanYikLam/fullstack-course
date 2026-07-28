// ============================================================
// 文件名: main.rs
// 功能: Rust 闭包（Closure）深入原理教学
// 对标 C++: Lambda 表达式 + std::function + 捕获机制
// 涵盖: 基础语法、捕获机制、Fn/FnMut/FnOnce、作为参数/返回值、性能分析
// ============================================================

// ============================================================
// 第一部分: 基础语法（与 C++ Lambda 对比）
// ============================================================
fn basic_syntax_demo() {
    println!("\n========== 1. 基础语法 ==========");

    // 1.1 最简单的闭包（无参数，无捕获）
    let hello = || println!("你好，世界！");
    hello(); // 调用

    // 1.2 带参数和返回值（类型自动推断）
    let add = |x, y| x + y;
    let result = add(3, 5);
    println!("add(3,5) = {}", result);

    // 1.3 显式类型标注（当编译器无法推断时）
    let mul = |x: i32, y: i32| -> i32 { x * y };
    println!("mul(4,6) = {}", mul(4, 6));

    // 1.4 多条语句的闭包（用花括号包围）
    let complex = |x| {
        let y = x + 10;
        y * 2
    };
    println!("complex(5) = {}", complex(5));

    // C++ 对比:
    // auto add = [](int x, int y) { return x + y; };
    // Rust 省去了 [] 和类型声明，更简洁！
}

// ============================================================
// 第二部分: 捕获机制（核心！）—— 对标 C++ 的 [&] [=] [move]
// Rust 的最大区别：编译器根据闭包体自动推断捕获方式！
// ============================================================
fn capture_mechanism_demo() {
    println!("\n========== 2. 捕获机制（引用 vs 可变引用 vs 移动）==========");

    let s = String::from("Hello");

    // 2.1 不可变捕获（&T）—— 闭包只读，相当于 C++ 的 [&]
    let read_only = || println!("读取: {}", s);
    read_only();
    read_only(); // 可以多次调用
    // s 仍然可用！
    println!("s 仍然有效: {}", s);

    // 2.2 可变捕获（&mut T）—— 闭包修改捕获的变量
    let mut s2 = String::from("World");
    {
        let mut modify = || {
            s2.push_str("!");
            println!("修改后: {}", s2);
        };
        modify();
        // modify 在这里被借用，不能再创建其他可变借用
        // 但我们可以继续调用 modify（只要不创建新的引用）
        modify();
    } // modify 离开作用域，可变借用结束
    println!("s2 最终值: {}", s2);

    // 2.3 值捕获（move）—— 强制转移所有权，对标 C++ 的按值捕获 + std::move
    let s3 = String::from("Move me");
    let consume = move || {
        // s3 的所有权被移动到闭包内部
        println!("闭包内部拥有: {}", s3);
        // s3 在这里被 drop（释放）
    };
    consume();
    // println!("{}", s3); // ❌ 编译错误！s3 已经被移动

    // 2.4 如果捕获的是 Copy 类型（如 i32），move 会拷贝而非移动
    let num = 42;
    let copy_closure = move || println!("数字: {}", num);
    copy_closure();
    println!("num 仍然可用（因为 i32 是 Copy）: {}", num); // ✅ 有效！
}

// ============================================================
// 第三部分: Fn / FnMut / FnOnce —— 闭包的三个 Trait（原理核心！）
// C++ 对标: operator() 的 const / non-const，以及移动语义
// ============================================================
fn fn_traits_demo() {
    println!("\n========== 3. Fn / FnMut / FnOnce 三大 Trait ==========");

    // 3.1 FnOnce —— 只能调用一次（因为调用了 consume 方法）
    // 类比 C++ 中按值捕获 unique_ptr 的 lambda，移动后不能再次调用
    let s_once = String::from("一次性");
    let consume = move || {
        let _ = s_once; // 注意：这里没有分号，实际上移动了 s_once
                       // 但因为 s_once 没有返回，实际编译时这行不会通过
                       // 为了演示，我们用 drop() 模拟消耗
    };
    // 实际上，真正的 FnOnce 需要闭包内部消耗掉捕获的变量。
    // 更典型的例子：
    let s_once2 = String::from("消耗品");
    let consume_once = || {
        drop(s_once2); // s_once2 被 drop，所有权被消耗
    };
    consume_once();
    // consume_once(); // ❌ 第二次调用会编译错误！因为闭包变成 FnOnce

    // 3.2 FnMut —— 可以修改捕获的变量（&mut self）
    // 类比 C++ 中非 const 的 operator()
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("计数器: {}", counter);
    };
    increment(); // 1
    increment(); // 2
    increment(); // 3

    // 3.3 Fn —— 不修改捕获的变量（&self），可以调用多次
    // 类比 C++ 中 const 的 operator()
    let greeting = String::from("你好");
    let say_hello = || println!("{}", greeting);
    say_hello();
    say_hello(); // 可以无限调用

    // 3.4 闭包自动实现哪个 Trait 取决于闭包体做了什么
    // 规则：
    // - 如果闭包移动了捕获的值（drop、move） -> FnOnce
    // - 如果闭包修改了捕获的值 -> FnMut（同时也实现了 FnOnce）
    // - 如果闭包只读 -> Fn（同时也实现了 FnMut 和 FnOnce）
    // 这三个 Trait 是继承关系：Fn : FnMut : FnOnce（类似 C++ 的继承层次）

    // 3.5 显式查看闭包的类型（使用 trait bound 来测试）
    fn test_fn<F: Fn()>(f: F) {
        f();
    }
    fn test_fn_mut<F: FnMut()>(mut f: F) {
        f();
    }
    fn test_fn_once<F: FnOnce()>(f: F) {
        f();
    }

    let x = 10;
    let read = || println!("{}", x); // Fn
    test_fn(read);
    test_fn_mut(read);
    test_fn_once(read);

    let mut y = 20;
    let mut write = || y += 1; // FnMut，不是 Fn
    // test_fn(write); // ❌ 编译错误：FnMut 不能当 Fn 用
    test_fn_mut(&mut write);
    test_fn_once(&mut write);

    let s = String::from("drop");
    let consume = || drop(s); // FnOnce
    // test_fn(consume); // ❌
    // test_fn_mut(consume); // ❌
    test_fn_once(consume);
}

// ============================================================
// 第四部分: 闭包的内存布局（幕后原理）
// 闭包本质上是一个匿名结构体，包含捕获的变量作为字段
// ============================================================
fn memory_layout_demo() {
    println!("\n========== 4. 内存布局（闭包就是结构体）==========");

    let msg = String::from("Hello");

    // 闭包捕获了 msg（不可变引用）
    let closure = || {
        println!("{}", msg);
    };

    // 闭包的大小 = 捕获变量的大小（在这里是 String 的大小）
    // 但因为是引用捕获，实际捕获的是 &String（指针大小）
    println!("闭包的大小: {} 字节", std::mem::size_of_val(&closure));
    println!("&String 的大小: {} 字节", std::mem::size_of::<&String>());

    // 手动模拟闭包的结构体（为了演示）
    struct MyClosure<'a> {
        msg: &'a String, // 捕获的引用
    }
    impl<'a> MyClosure<'a> {
        fn call(&self) {
            println!("{}", self.msg);
        }
    }
    let my_closure = MyClosure { msg: &msg };
    my_closure.call();
    println!("手动结构体的大小: {} 字节", std::mem::size_of_val(&my_closure));

    // 结论：闭包 == 实现了某个 Fn Trait 的匿名结构体
    // 零运行时开销！
}

// ============================================================
// 第五部分: 闭包作为参数（静态派发 vs 动态派发）
// C++ 对标: template 接受 lambda vs std::function
// ============================================================
fn parameter_demo() {
    println!("\n========== 5. 闭包作为参数 ==========");

    // 5.1 使用泛型 + Trait Bound（静态派发，零开销，推荐）
    // 相当于 C++ 的 template<typename F> void apply(F f)
    fn apply_static<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let double = |x| x * 2;
    let result1 = apply_static(double, 5);
    println!("静态派发: {}", result1);

    // 5.2 使用 impl Trait（语法糖，同样静态派发）
    fn apply_impl(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }
    let triple = |x| x * 3;
    let result2 = apply_impl(triple, 5);
    println!("impl Trait: {}", result2);

    // 5.3 使用动态派发 Box<dyn Fn>（有运行时开销，对应 std::function）
    fn apply_dynamic(f: Box<dyn Fn(i32) -> i32>, x: i32) -> i32 {
        f(x)
    }
    let square = |x| x * x;
    let result3 = apply_dynamic(Box::new(square), 5);
    println!("动态派发 (Box): {}", result3);

    // 5.4 使用 &dyn Fn（引用形式，避免 Box 分配）
    fn apply_dyn_ref(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }
    let negate = |x| -x;
    let result4 = apply_dyn_ref(&negate, 5);
    println!("动态派发 (引用): {}", result4);
}

// ============================================================
// 第六部分: 闭包作为返回值（返回闭包的函数）
// C++ 最难处理的地方，Rust 通过 impl Trait 和 Box 解决
// ============================================================
fn return_demo() {
    println!("\n========== 6. 闭包作为返回值 ==========");

    // 6.1 返回单一具体类型的闭包（使用 impl Fn）
    // 注意：所有返回值必须是同一个闭包类型
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        // 返回一个捕获了 x 的闭包
        move |y| x + y
    }

    let add5 = make_adder(5);
    let add10 = make_adder(10);
    println!("add5(3) = {}", add5(3));
    println!("add10(3) = {}", add10(3));

    // 6.2 返回闭包引用（借用，需要生命周期标注）
    fn make_multiplier<'a>(x: i32) -> Box<dyn Fn(i32) -> i32 + 'a> {
        Box::new(move |y| x * y)
    }

    let mul3 = make_multiplier(3);
    println!("mul3(7) = {}", mul3(7));

    // 6.3 返回闭包，不能使用 &dyn，因为返回的闭包是局部变量，必须有 'static 或 Box

    // 6.4 返回多个不同闭包（比如基于条件返回不同的闭包）
    // 必须用 Box<dyn Fn>，因为 impl Fn 要求返回类型固定
    fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
        match op {
            "add" => Box::new(|a, b| a + b),
            "sub" => Box::new(|a, b| a - b),
            "mul" => Box::new(|a, b| a * b),
            _ => Box::new(|a, _| a), // 默认返回第一个参数
        }
    }

    let add_op = make_operation("add");
    let sub_op = make_operation("sub");
    println!("add_op(10, 5) = {}", add_op(10, 5));
    println!("sub_op(10, 5) = {}", sub_op(10, 5));
}

// ============================================================
// 第七部分: 实战 —— 迭代器 + 闭包（最黄金组合）
// ============================================================
fn iterator_closure_demo() {
    println!("\n========== 7. 实战: 迭代器 + 闭包 ==========");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 7.1 复杂流水线：过滤偶数 -> 乘以3 -> 求和
    let sum: i32 = data
        .iter()
        .filter(|&&x| x % 2 == 0) // 闭包过滤偶数
        .map(|&x| x * 3)          // 闭包映射
        .sum();
    println!("偶数 * 3 求和: {}", sum);

    // 7.2 统计字符串长度（使用闭包计算）
    let words = vec!["Rust", "C++", "Python", "Java"];
    let total_len: usize = words.iter().map(|w| w.len()).sum();
    println!("总字符数: {}", total_len);

    // 7.3 使用闭包进行分组（按长度分组）
    use std::collections::HashMap;
    let mut groups = HashMap::new();
    for word in &words {
        let len = word.len();
        groups
            .entry(len)
            .or_insert_with(Vec::new) // 闭包创建新 Vec
            .push(*word);
    }
    println!("按长度分组: {:?}", groups);
}

// ============================================================
// 第八部分: 性能与最佳实践（C++ 程序员必须知道的坑）
// ============================================================
fn performance_best_practice_demo() {
    println!("\n========== 8. 性能与最佳实践 ==========");

    // 8.1 闭包捕获大对象时的移动 vs 引用
    let big_string = String::from("x".repeat(1000));

    // 不好：移动了 big_string（因为 move）
    let bad = move || {
        println!("长度: {}", big_string.len());
    };
    bad();
    // big_string 已失效

    // 好：使用引用（如果只需要读）
    let big_string2 = String::from("x".repeat(1000));
    let good = || {
        println!("长度: {}", big_string2.len()); // 只读，捕获 &String
    };
    good();
    println!("big_string2 仍然有效: {}", big_string2); // ✅

    // 8.2 避免在循环中创建闭包（如果闭包很复杂，可能影响性能）
    // 但要避免不必要的分配
    let nums = vec![1, 2, 3, 4, 5];
    let factor = 10;
    // 好：闭包在循环外定义（只创建一次）
    let multiply = |x| x * factor;
    let result: Vec<i32> = nums.iter().map(|&x| multiply(x)).collect();
    println!("重用闭包: {:?}", result);

    // 8.3 使用 Fn 约束来保证不修改捕获的值（提高代码可读性）
    fn process_data<F>(f: F, data: &[i32]) -> Vec<i32>
    where
        F: Fn(i32) -> i32, // 只读，不能修改
    {
        data.iter().map(|&x| f(x)).collect()
    }

    let double = |x| x * 2;
    let processed = process_data(double, &nums);
    println!("process_data 处理: {:?}", processed);
}

// ============================================================
// 第九部分: 终极原理 —— 闭包捕获是什么？编译器如何转换？
// 这里手动模拟编译器对闭包的脱糖（Desugaring）
// ============================================================
fn desugaring_demo() {
    println!("\n========== 9. 终极原理: 编译器脱糖（Desugaring）==========");

    // 原始闭包
    let factor = 10;
    let closure = |x| x * factor;
    println!("闭包结果: {}", closure(5));

    // 编译器脱糖后的等价代码（手动模拟）：
    struct __Closure {
        factor: i32, // 捕获的变量作为字段
    }

    impl __Closure {
        // Fn 的 call 方法（不可变引用）
        fn call(&self, x: i32) -> i32 {
            x * self.factor
        }
    }

    let __closure = __Closure { factor: 10 };
    println!("脱糖后结果: {}", __closure.call(5));

    // 对于 FnMut（可变闭包），脱糖为 call_mut(&mut self)
    let mut counter = 0;
    let mut mut_closure = || {
        counter += 1;
        counter
    };
    println!("FnMut 调用1: {}", mut_closure());
    println!("FnMut 调用2: {}", mut_closure());

    // 脱糖模拟：
    struct __ClosureMut {
        counter: i32,
    }
    impl __ClosureMut {
        fn call_mut(&mut self) -> i32 {
            self.counter += 1;
            self.counter
        }
    }
    let mut __closure_mut = __ClosureMut { counter: 0 };
    println!("脱糖 FnMut 调用1: {}", __closure_mut.call_mut());
    println!("脱糖 FnMut 调用2: {}", __closure_mut.call_mut());

    // 对于 FnOnce（move + drop），脱糖为 call_once(self)
    let s = String::from("drop");
    let once_closure = || {
        drop(s);
    };
    // once_closure(); // 只能调用一次

    // 脱糖模拟：
    struct __ClosureOnce {
        s: String,
    }
    impl __ClosureOnce {
        fn call_once(self) {
            drop(self.s);
        }
    }
    let __once = __ClosureOnce { s: String::from("drop") };
    __once.call_once();
    // __once.call_once(); // ❌ 编译错误，self 已经被消耗
}

// ============================================================
// 主函数入口
// ============================================================
fn main() {
    println!("🔥 Rust 闭包（Closure）深入原理");
    println!("（请对照 C++ 的 Lambda + std::function 理解）\n");

    basic_syntax_demo();
    capture_mechanism_demo();
    fn_traits_demo();
    memory_layout_demo();
    parameter_demo();
    return_demo();
    iterator_closure_demo();
    performance_best_practice_demo();
    desugaring_demo();

    println!("\n✅ 所有闭包演示完成！");
    println!("💡 记住黄金法则:");
    println!("  1. 闭包自动推断捕获方式（&, &mut, move）——相信编译器！");
    println!("  2. Fn 只读（最宽松），FnMut 可修改，FnOnce 只能调用一次");
    println!("  3. 闭包默认是零成本抽象（静态派发），除非用 Box<dyn>");
    println!("  4. move 关键字强制转移所有权，在需要跨线程传递时极其有用");
    println!("  5. 闭包本质上就是实现 Fn Trait 的匿名结构体！");
}