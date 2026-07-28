// =============================================
// 第一部分：最经典的“悬垂引用”错误（C++ UB vs Rust 编译失败）
// C++ 写法: const std::string& get_name() { std::string s = "hi"; return s; } // 返回局部引用，UB！
// =============================================

// ❌ 下面的代码如果解开注释，Rust 会无情地拒绝编译：
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 错误：返回局部变量的引用，s 在此函数结束会被释放
// }
// 编译器报错：`s` does not live long enough

// ✅ 正确做法：把所有权返回出来，或者传入外部引用
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 移动所有权给调用方，安全！
}

// =============================================
// 第二部分：生命周期注解（显式标注 'a）
// 场景：比较两个字符串，返回较长的那个（C++ 你会返回 const char* 或 const string&）
// 问题：编译器不知道返回的引用应该活多久？属于 x 还是 y？
// =============================================

// 👇 这里的 'a 是个“泛型生命周期参数”
// 意思是：返回的引用，和传入的 x、y 存活时间中的“较小者”一样长。
// 中文翻译：返回值必须至少和 x、y 中生命周期最短的那个一样长。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 如果你不写 'a，编译器会报错：missing lifetime specifier
// 因为 Rust 不知道你到底要返回哪一个输入的生命周期。

// =============================================
// 第三部分：结构体中的生命周期（C++ 类持有引用时必须格外小心）
// =============================================

// 定义一个结构体，它持有一个字符串切片引用。
// 必须告诉编译器：这个结构体的实例不能比它引用的字符串活得更久。
struct Excerpt<'a> {
    part: &'a str, // part 必须和结构体实例具有相同的生命周期参数 'a
}

impl<'a> Excerpt<'a> {
    // 方法中默认会省略生命周期，但这里显式写出来让你看清：
    // 返回值 &str 的生命周期与 self 绑定（即 'a）
    fn get_part(&self) -> &str {
        self.part
    }
    
    // 更复杂的例子：传入另一个引用，返回其中一个
    fn announce<'b>(&'a self, announcement: &'b str) -> &'a str 
    where 
        'a: 'b, // 表示 'a 比 'b 活得长（或一样长），确保 self.part 可以安全返回
    {
        println!("重要通知：{}", announcement);
        self.part
    }
}

// =============================================
// 第四部分：生命周期省略规则（Elision）—— 让你不用每次都写 'a
// Rust 编译器有一套默认规则，大部分常用情况可以省略。
// =============================================

// 规则1：每个输入引用默认有自己的生命周期。
// 规则2：如果只有一个输入引用，输出的生命周期等于它。
// 规则3：如果有 &self 或 &mut self，输出的生命周期等于 self。

// 因此，这个常见函数你不需要写任何生命周期注解：
fn first_word(s: &str) -> &str {
    // 编译器内部自动补全为：fn first_word<'a>(s: &'a str) -> &'a str
    &s[..1] // 取第一个字符（演示而已，实际取单词可能更复杂）
}

// 但如果是多个输入引用，编译器就无法省略了，你必须显式写！
// fn bad_example(a: &str, b: &str) -> &str // ❌ 编译错误，必须写 'a

// =============================================
// 第五部分：'static 生命周期（相当于 C 的全局/静态存储区）
// =============================================

// 'static 表示这个引用在整个程序运行期间都有效。
// 字符串字面量（"Hello"）就是 &'static str。
fn static_demo() -> &'static str {
    let s: &'static str = "I live forever!";
    s
}

// 注意：滥用 'static 来修复编译错误是坏习惯！
// 如果是运行时分配的 String，千万不要强行标注 'static（除非用 Box::leak 等刻意操作）

// =============================================
// 主函数演示
// =============================================

fn main() {
    // ---- 演示生命周期注解函数 ----
    let string1 = String::from("Rust is awesome");
    let string2 = "Short";
    
    // 注意：result 的生命周期取 string1 和 string2 中较短的那个（即 string2，因为是静态字面量）
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是：{}", result);
    
    // ---- 演示结构体生命周期 ----
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("找不到句号");
    let excerpt = Excerpt { part: first_sentence };
    // 此时 novel 必须比 excerpt 活得久（确实如此，因为 excerpt 在 novel 作用域内）
    println!("摘录内容：{}", excerpt.get_part());
    
    // ---- 演示生命周期省略 ----
    let word = first_word("Hello World");
    println!("第一个字符：{}", word);
    
    // ---- 演示 'static ----
    let forever = static_demo();
    println!("静态字符串：{}", forever);
    
    // ---- 演示借用检查器拦截错误（运行时不会崩溃，因为编译就拦住了） ----
    let result_from_longest;
    {
        let short_lived = String::from("临时");
        // 注意：这里 longest 返回的引用生命周期绑定于 short_lived（较短的那个）
        // 但如果 short_lived 结束后，result_from_longest 还想用，编译器会阻止！
        result_from_longest = longest(string1.as_str(), short_lived.as_str());
        println!("作用域内打印：{}", result_from_longest); // ✅ 在作用域内安全
    }
    // println!("作用域外打印：{}", result_from_longest); // ❌ 解开注释会编译失败！
    // 因为 result_from_longest 引用的数据（short_lived）已经被释放。
    println!("上面那行如果解开注释会报错，Rust 阻止了悬垂引用！");
}