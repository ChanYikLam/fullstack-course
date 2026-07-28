// ============================================================
// 文件名: main.rs
// 功能: Rust 错误处理完全教学 —— 对标 C++ 的 optional / expected / 异常
// 涵盖: Option, Result, ? 操作符, 自定义错误, 错误组合
// ============================================================

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::fmt;

// ============================================================
// 第一部分: Option<T> —— 对标 C++17 的 std::optional
// 场景: 值可能存在也可能不存在（如查找元素、除零运算）
// ============================================================
fn option_demo() {
    println!("\n========== 1. Option<T> （对标 std::optional）==========");

    // 1.1 创建 Some 和 None
    let some_num: Option<i32> = Some(42);
    let none_num: Option<i32> = None;

    // 1.2 安全解包：match
    match some_num {
        Some(val) => println!("some_num 中有值: {}", val),
        None => println!("没有值"),
    }

    // 1.3 常用解包方法（危险操作，类比 C++ 的 *optional 但不检查）
    // unwrap: 如果是 None 则 panic（明确告诉程序 "这里一定会有值"）
    let value = some_num.unwrap(); // 安全，因为有值
    println!("unwrap 得到: {}", value);
    // let bad = none_num.unwrap(); // ❌ 这行会 panic！程序崩溃

    // 1.4 使用 expect 提供自定义错误信息（比 unwrap 更好）
    // 这行会 panic 并打印 "计算失败，因为除数为零"
    // none_num.expect("计算失败，因为除数为零");

    // 1.5 安全解包：使用 unwrap_or（提供默认值）
    let val1 = none_num.unwrap_or(0);
    println!("None 时提供默认值: {}", val1);

    // 1.6 安全解包：使用 unwrap_or_else（延迟计算默认值）
    let val2 = none_num.unwrap_or_else(|| {
        println!("执行了默认值计算！");
        100
    });
    println!("延迟计算的默认值: {}", val2);

    // 1.7 实际应用场景：在 Vec 中查找元素（找不到返回 None）
    let vec = vec![10, 20, 30];
    let found = vec.iter().find(|&&x| x == 20);
    match found {
        Some(&val) => println!("找到了: {}", val),
        None => println!("未找到"),
    }
}

// ============================================================
// 第二部分: Result<T, E> —— 对标 C++23 的 std::expected
// 场景: 操作可能成功（Ok）或失败（Err），如文件读取、网络请求
// ============================================================
fn result_demo() {
    println!("\n========== 2. Result<T, E> （对标 std::expected）==========");

    // 2.1 模拟解析字符串为数字
    let good: Result<i32, ParseIntError> = "42".parse();
    let bad: Result<i32, ParseIntError> = "abc".parse();

    // 2.2 使用 match 处理
    match good {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }

    // 2.3 使用 unwrap / expect（失败则 panic）
    let num = good.unwrap(); // 成功
    println!("直接 unwrap: {}", num);
    // bad.unwrap(); // ❌ 这行会 panic！

    // 2.4 使用 unwrap_or 提供默认值
    let num_or = bad.unwrap_or(0);
    println!("解析失败时使用默认值: {}", num_or);

    // 2.5 使用 ? 操作符（见下一部分，这里仅演示 match 处理）
    // 真实场景中，通常用 ? 向上传播错误，而不是在这里 unwrap。
}

// ============================================================
// 第三部分: ? 操作符 —— Rust 最强大的错误传播工具
// 对标 C++ 的 throw 异常，但没有运行时开销（纯编译期展开）
// 核心: 在函数中，如果 Result 是 Err，立即 return Err；如果是 Ok，解包并继续。
// ============================================================

// 3.1 辅助函数：读取文件内容（返回 Result<String, io::Error>）
// 注意：这个函数用了 ?，如果打开或读取失败，会直接向上返回 Err
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    // 如果 File::open 失败，? 会立即将 Err 返回给调用方
    let mut file = File::open(filename)?;
    let mut content = String::new();
    // 如果 read_to_string 失败，? 也会立即返回 Err
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 3.2 链式调用：读取文件 + 解析第一个数字
// 这里有多种错误类型（io::Error 和 ParseIntError），需要处理
fn parse_first_number_from_file(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // 方式1：使用 ? 自动传播 io::Error
    let content = read_file_content(filename)?;
    // 按空白分割，取第一个单词
    let first_word = content.split_whitespace().next().ok_or("文件为空")?;
    // 解析为 i32，ParseIntError 通过 ? 自动向上传播
    // 但 ? 要求错误类型一致，这里用 map_err 将 ParseIntError 转为 Box<dyn Error>
    let number = first_word.parse::<i32>()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(number)
}

// 3.3 演示 ? 操作符的威力（与 C++ 对比）
fn question_mark_demo() {
    println!("\n========== 3. ? 操作符（对标 throw / 提前返回）==========");

    // 创建一个临时文件（仅演示，真实场景不要这样写）
    // 为了演示，我们先写一个文件
    let filename = "test.txt";
    std::fs::write(filename, "42\nhello world").unwrap();

    // 调用我们上面定义的函数
    match parse_first_number_from_file(filename) {
        Ok(num) => println!("从文件解析到的第一个数字是: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }

    // 清理文件
    std::fs::remove_file(filename).unwrap();

    // 如果文件不存在，? 会优雅地返回 Err
    match parse_first_number_from_file("nonexistent.txt") {
        Ok(num) => println!("数字: {}", num),
        Err(e) => println!("预期中的失败（文件不存在）: {}", e),
    }

    // C++ 对比：
    // C++ 中你需要写: 
    // try { auto content = read_file(); } catch(...) { return; }
    // 或者 if (!file.is_open()) { return error; }
    // Rust 的 ? 把这种样板代码压缩到极致！
}

// ============================================================
// 第四部分: 组合错误 —— 将 Option 和 Result 互相转换
// ============================================================
fn combination_demo() {
    println!("\n========== 4. Option 与 Result 互转 ==========");

    // 4.1 将 Option 转为 Result（ok_or / ok_or_else）
    let opt: Option<i32> = Some(42);
    let result_from_opt = opt.ok_or("值不存在").unwrap();
    println!("Option -> Result: {}", result_from_opt);

    let opt_none: Option<i32> = None;
    let result_from_none = opt_none.ok_or_else(|| "自定义错误信息".to_string());
    match result_from_none {
        Ok(v) => println!("得到值: {}", v),
        Err(e) => println!("转为 Err: {}", e),
    }

    // 4.2 将 Result 转为 Option（ok() / err()）
    let res: Result<i32, &str> = Ok(100);
    let opt_from_res = res.ok(); // 得到 Some(100)，忽略 Err
    println!("Result -> Option: {:?}", opt_from_res);

    let res_err: Result<i32, &str> = Err("错误");
    let opt_from_err = res_err.ok(); // 得到 None
    println!("失败的 Result -> Option: {:?}", opt_from_err);
}

// ============================================================
// 第五部分: 自定义错误类型（对标 C++ 的自定义异常类）
// Rust 推荐使用枚举来组织错误，并实现 std::error::Error trait
// ============================================================

// 定义一个自定义错误枚举
#[derive(Debug)]
enum MyAppError {
    IoError(io::Error),                     // 包装标准库的 IO 错误
    ParseError(ParseIntError),              // 包装解析错误
    CustomError(String),                    // 自定义信息
    EmptyData,                              // 无数据错误
}

// 必须实现 Display trait（用于用户友好的输出）
impl fmt::Display for MyAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyAppError::IoError(e) => write!(f, "IO 错误: {}", e),
            MyAppError::ParseError(e) => write!(f, "解析错误: {}", e),
            MyAppError::CustomError(msg) => write!(f, "自定义错误: {}", msg),
            MyAppError::EmptyData => write!(f, "数据为空"),
        }
    }
}

// 实现 std::error::Error trait（允许转换为通用错误）
impl std::error::Error for MyAppError {}

// 实现从 io::Error 自动转换为 MyAppError（方便 ? 操作符）
impl From<io::Error> for MyAppError {
    fn from(error: io::Error) -> Self {
        MyAppError::IoError(error)
    }
}

// 实现从 ParseIntError 自动转换为 MyAppError
impl From<ParseIntError> for MyAppError {
    fn from(error: ParseIntError) -> Self {
        MyAppError::ParseError(error)
    }
}

// 使用自定义错误的函数
fn parse_custom(filename: &str) -> Result<i32, MyAppError> {
    let content = read_file_content(filename)?; // ? 自动将 io::Error 转为 MyAppError
    let first_word = content.split_whitespace().next().ok_or(MyAppError::EmptyData)?;
    // ? 自动将 ParseIntError 转为 MyAppError
    let number = first_word.parse::<i32>()?;
    Ok(number)
}

fn custom_error_demo() {
    println!("\n========== 5. 自定义错误类型 ==========");

    let filename = "custom_test.txt";
    std::fs::write(filename, "not_a_number").unwrap();

    match parse_custom(filename) {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("自定义错误捕获: {}", e), // 这里会打印 "解析错误: ..."
    }

    // 测试文件不存在
    match parse_custom("missing.txt") {
        Ok(num) => println!("{}", num),
        Err(e) => println!("文件不存在时的错误: {}", e),
    }

    std::fs::remove_file(filename).unwrap();
}

// ============================================================
// 第六部分: 链式调用与组合器（map, and_then, or_else）
// 对标 C++ 的 std::optional 和 std::expected 的变换操作
// ============================================================
fn combinator_demo() {
    println!("\n========== 6. 组合器（map / and_then / or_else）==========");

    // 6.1 Option 的 map（转换内部值）
    let opt = Some(5);
    let mapped = opt.map(|x| x * 2);
    println!("map 5*2 = {:?}", mapped); // Some(10)

    let none: Option<i32> = None;
    let mapped_none = none.map(|x| x * 2);
    println!("map None = {:?}", mapped_none); // None

    // 6.2 Option 的 and_then（返回 Option 的链式调用，类似 flat_map）
    let opt2 = Some(10);
    let result = opt2.and_then(|x| {
        if x > 5 {
            Some(x * 2)
        } else {
            None
        }
    });
    println!("and_then (x>5) = {:?}", result); // Some(20)

    let opt3 = Some(3);
    let result2 = opt3.and_then(|x| {
        if x > 5 {
            Some(x * 2)
        } else {
            None
        }
    });
    println!("and_then (x<=5) = {:?}", result2); // None

    // 6.3 Result 的 map_err（转换错误类型）
    let res: Result<i32, &str> = Err("原始错误");
    let res_mapped = res.map_err(|e| format!("转换后: {}", e));
    println!("map_err: {:?}", res_mapped);

    // 6.4 实际应用：解析字符串 -> 转为整数 -> 判断正负
    let input = "42";
    let output = input
        .parse::<i32>()
        .ok()
        .and_then(|n| if n > 0 { Some(n) } else { None })
        .map(|n| n * 10)
        .unwrap_or(0);
    println!("组合器链式调用结果: {}", output);

    // 负数的例子
    let input_neg = "-5";
    let output_neg = input_neg
        .parse::<i32>()
        .ok()
        .and_then(|n| if n > 0 { Some(n) } else { None })
        .map(|n| n * 10)
        .unwrap_or(0);
    println!("负数的链式调用结果（返回默认0）: {}", output_neg);
}

// ============================================================
// 第七部分: main 函数也可以返回 Result（简化错误处理）
// 类比 C++ 的 int main() { try { ... } catch(...) { return 1; } }
// ============================================================

// 这是个特殊的 main，可以返回 Result，如果有错误会打印并退出
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Rust 错误处理完全指南\n");

    option_demo();
    result_demo();
    question_mark_demo();
    combination_demo();
    custom_error_demo();
    combinator_demo();

    // 演示 main 返回 Result：故意读取一个文件，如果失败会提前返回
    println!("\n========== 7. main 返回 Result ==========");
    let content = read_file_content("maybe_not_exist.txt")?; // 如果失败，main 直接返回 Err
    println!("文件内容: {}", content); // 这行不会执行

    // 这行不会执行，因为上面已经返回了
    // Ok(())

    // 但为了程序能完整运行并看到上面的所有输出，我们手动 return Ok
    // 实际上，上面 read_file_content 会触发 main 提前返回并打印错误。
    // 为了演示完整流程，我们可以注释掉那行，或在末尾显式返回。
    // 下面我 return Ok 以确保所有演示都被看到，但你可以自行取消上面的注释测试。
    
    println!("\n✅ 所有错误处理演示完成！");
    println!("💡 记住黄金法则:");
    println!("  1. 能用 ? 就别用 unwrap（除非你绝对确定不会出错）");
    println!("  2. 自定义错误类型时，实现 From 特征配合 ? 使用最爽");
    println!("  3. 库代码用自定义错误（thiserror），应用代码用 anyhow");
    Ok(())
}