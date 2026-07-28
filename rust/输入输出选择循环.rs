use std::io; // 引入输入输出库（类似 C++ 的 #include <iostream>）
use std::cmp::Ordering; // 用于比较结果（Less, Equal, Greater）

// =============================================
// 第一部分：输入（Input） 与 输出（Output）
// C++ 对比: std::cin >> x;  vs  std::cout << x << std::endl;
// =============================================

fn io_demo() {
    // ---- 输出（Print） ----
    // 1. 普通字符串（不带换行的用 print!，带换行的用 println!）
    print!("请输入你的年龄: "); // 不换行，类似 std::cout << "请输入..." << std::flush;
    
    // 2. 格式化输出（类似 printf 或 std::format，但编译时检查）
    let name = "Alice";
    let score = 95.5;
    // {} 是默认占位符，{:?} 是调试占位符（打印数组、结构体等），{:.2} 保留两位小数
    println!("你好，{}！你的分数是 {:.2} 分", name, score); 
    
    // ---- 输入（Input） ----
    // Rust 标准做法：先创建一个可变 String 缓冲区，再读取整行
    let mut input_buffer = String::new(); // 类似 C++ 的 std::string buffer;
    
    // read_line 会读取用户输入的整行（包含换行符），追加到 buffer 中
    // 返回值是 Result（类似 C++ 的 std::expected 或异常），用 expect 粗暴处理错误
    io::stdin()
        .read_line(&mut input_buffer) // 传入可变引用，类似 C++ 的 std::getline(std::cin, buffer);
        .expect("读取输入失败！"); // 如果失败，程序直接 panic（崩溃）并打印信息
    
    // ⚠️ 重要！read_line 会把末尾的换行符 \n 也读进来，必须去除
    // trim() 去掉首尾空白字符，类似 C++ 的 boost::trim 或手动检查
    let trimmed_input = input_buffer.trim();
    
    // 将字符串解析为数字（必须显式处理，不像 C++ 的 cin >> 自动转换）
    // parse::<i32>() 告诉编译器解析成 32 位整数
    let age: i32 = trimmed_input
        .parse()
        .expect("请输入一个有效的数字！"); // 如果解析失败，程序崩溃
    
    // 如果不想让程序崩溃，可以用 match 处理（后面会讲），这里先用 expect
    println!("你输入的年龄是：{}", age);
}

// =============================================
// 第二部分：选择结构（Selection）
// 涵盖：if-else，以及 Rust 独门绝技 match（对应 C++ 的 switch 但强大百倍）
// =============================================

fn selection_demo(age: i32) {
    // ---- 1. if-else（与 C++ 几乎一样，但条件必须是 bool） ----
    // 注意：不能写 if (age) 必须写 if age != 0
    if age < 18 {
        println!("[if] 未成年人禁止入内");
    } else if age >= 18 && age < 60 {
        println!("[if] 欢迎中青年用户");
    } else {
        println!("[if] 老年用户，祝您健康！");
    }

    // ⚠️ Rust 的 if 是表达式（可以返回值），替代 C++ 的三元运算符（? :）
    // C++: int status = (age >= 18) ? 1 : 0;
    let status = if age >= 18 { 1 } else { 0 };
    println!("状态码（18岁以上为1）：{}", status);

    // ---- 2. match（超级增强版 switch） ----
    // C++ 的 switch 只能匹配整数/枚举，而且容易忘记 break。
    // Rust 的 match 必须穷尽所有可能（否则编译报错），且可以匹配范围、字面量、甚至解构。
    println!("\n--- match 演示 ---");
    
    // 场景：根据年龄段给出特定提示
    match age {
        // 匹配单个值
        0 => println!("你刚出生吗？"),
        // 匹配范围（包含右边界 ..=）
        1..=12 => println!("童年时光，好好玩耍"),
        13..=17 => println!("青春期，学点 Rust 吧！"),
        // 匹配多个值（用 | 分隔）
        18 | 20 | 22 => println!("刚成年的黄金年龄"),
        // 匹配范围 + 绑定值到变量（@ 语法）
        age @ 25..=30 => println!("区间匹配并绑定：年龄 {} 岁，正是事业上升期", age),
        // 匹配剩余所有情况（必须用 _ 占位符，类似 C++ 的 default:）
        _ => println!("其他年龄段（{} 岁）", age),
    }

    // ---- 3. match 作为表达式返回值（极其常用） ----
    // C++ 中你想根据枚举返回字符串，通常用 switch 赋值或 map。
    let category = match age {
        0..=17 => "Minor",
        18..=64 => "Adult",
        _ => "Senior",
    };
    println!("根据年龄分类：{}", category);
}

// =============================================
// 第三部分：循环结构（Loops）
// 涵盖：loop（无限循环）、while、for（彻底告别 C 风格索引）
// =============================================

fn loop_demo() {
    println!("\n--- 循环演示 ---");

    // ---- 1. loop（无限循环，相当于 C++ 的 while(true) { ... }） ----
    // 特殊能力：可以用 break 返回值，这在 C++ 里做不到（除非用 lambda + return）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            // break 后面跟的值会作为 loop 表达式的返回值
            break counter * 10; 
        }
    };
    println!("[loop] 计数器在 5 时跳出，返回值: {}", result); // 输出 50

    // ---- 2. while（条件循环，与 C++ 一致，但条件必须是 bool） ----
    let mut num = 3;
    while num > 0 {
        println!("[while] 倒计时: {}", num);
        num -= 1; // 注意：Rust 不支持 num-- 自减运算符（为了明确性），必须写 num -= 1
    }

    // ---- 3. for（Rust 的灵魂迭代！） ----
    // C++ 程序员最爱写的：for (int i = 0; i < 10; i++) { ... }
    // Rust 完全抛弃这种写法，改用 范围（Range） + 迭代器
    
    // 3.1 正向遍历范围（左闭右开）：0,1,2,3,4
    print!("[for] 0..5 => ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // 3.2 正向遍历包含右边界（左闭右闭）：0,1,2,3,4,5
    print!("[for] 0..=5 => ");
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();

    // 3.3 反向遍历（利用 rev() 迭代器适配器）
    print!("[for] 反转 (5..0) => ");
    for i in (0..5).rev() {
        print!("{} ", i); // 输出 4 3 2 1 0
    }
    println!();

    // 3.4 遍历数组/集合（这才是 Rust 最常用的方式，彻底告别下标越界）
    let arr = [10, 20, 30, 40];
    print!("[for] 遍历数组 => ");
    for element in arr.iter() { // .iter() 产生不可变引用，类似 C++ 的 for(auto &e : arr)
        print!("{} ", element);
    }
    println!();

    // 3.5 遍历集合并获取索引（类似 C++ 枚举，但用 enumerate()）
    print!("[for] 带索引遍历 => ");
    for (index, value) in arr.iter().enumerate() {
        print!("[{}]={} ", index, value);
    }
    println!();

    // ---- 4. 循环控制：break 和 continue（与 C++ 一致） ----
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        if i > 7 {
            break; // 大于7就停止
        }
        print!("{} ", i); // 输出 1 3 5 7
    }
    println!();
}

// =============================================
// 附加彩蛋：用 match 处理输入错误（优雅处理 Result，而不是 expect）
// =============================================

fn safe_input_demo() {
    println!("\n--- 安全输入演示（不崩溃） ---");
    let mut buffer = String::new();
    println!("请输入一个数字（输入字母也不会崩溃）：");
    
    io::stdin().read_line(&mut buffer).expect("读取失败");
    let trimmed = buffer.trim();
    
    // 用 match 优雅处理解析成功或失败
    match trimmed.parse::<i32>() {
        Ok(number) => println!("你输入的数字是：{}", number),
        Err(_) => println!("错误：你输入的不是一个合法整数！"),
    }
}

// =============================================
// 主函数入口
// =============================================

fn main() {
    // 注意：为了快速展示，io_demo 里用了 expect，如果输入不是数字会崩溃。
    // 为了安全演示，我注释掉 io_demo，改成 safe_input_demo。
    // 如果你想体验输入输出，可以把下面这行解开注释，并注释掉 safe_input_demo。
     io_demo(); 
    
    //safe_input_demo(); // 演示安全的输入解析

    // 因为 io_demo 已经被注释，这里我们模拟一个年龄来演示选择结构
    println!("\n--- 模拟年龄 25 进行选择结构测试 ---");
    selection_demo(25);
    
    loop_demo();
}