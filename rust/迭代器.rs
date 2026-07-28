// ============================================================
// 文件名: main.rs
// 功能: Rust 迭代器（Iterator）完全教学
// 对标 C++: std::vector::iterator, std::transform, std::accumulate
// ============================================================

use std::collections::HashMap;

// ============================================================
// 第一部分: 迭代器的三种形态（iter, iter_mut, into_iter）
// 核心: 所有权决定用哪种！
// ============================================================
fn iterator_types_demo() {
    println!("\n========== 1. 迭代器的三种形态 ==========");

    let v = vec![10, 20, 30];

    // 1.1 iter() —— 产生不可变引用 &T（最常用）
    println!("iter() 产生 &i32:");
    for item in v.iter() {
        print!("{} ", item); // item 是 &i32
    }
    println!();
    // v 仍然有效！

    // 1.2 iter_mut() —— 产生可变引用 &mut T（用于修改）
    let mut v_mut = vec![1, 2, 3];
    for item in v_mut.iter_mut() {
        *item *= 10;
    }
    println!("iter_mut() 修改后: {:?}", v_mut);

    // 1.3 into_iter() —— 取得所有权（消耗容器），产生 T
    let v_owned = vec!["a", "b", "c"];
    for item in v_owned.into_iter() {
        print!("{} ", item); // item 是 &str，但所有权已转移到循环
    }
    println!();
    // println!("{:?}", v_owned); // ❌ 编译错误：v_owned 已被移动

    // 语法糖：for 循环默认调用 into_iter()
    let v_sugar = vec![100, 200, 300];
    for item in &v_sugar { // 等价于 v_sugar.iter()
        print!("{} ", item);
    }
    println!();
    for item in &mut v_sugar { // 等价于 v_sugar.iter_mut()
        // 这里不能修改，因为 v_sugar 不是 mut，仅演示类型
    }
    for item in v_sugar { // 等价于 v_sugar.into_iter()
        print!("{} ", item);
    }
    println!();
}

// ============================================================
// 第二部分: 核心 Trait —— Iterator
// 只有 next() 方法是必须实现的，其他都是默认提供
// ============================================================
fn iterator_trait_demo() {
    println!("\n========== 2. Iterator Trait 与 next() ==========");

    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    // next() 返回 Option<&T>，每次调用消费一个元素
    println!("iter.next() = {:?}", iter.next()); // Some(1)
    println!("iter.next() = {:?}", iter.next()); // Some(2)
    println!("iter.next() = {:?}", iter.next()); // Some(3)
    println!("iter.next() = {:?}", iter.next()); // None

    // 手动实现一个简单迭代器（计数到 5）
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Self {
            Counter { count: 0, max }
        }
    }

    // 实现 Iterator trait
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter::new(5);
    for num in counter {
        print!("{} ", num); // 输出 1 2 3 4 5
    }
    println!();
}

// ============================================================
// 第三部分: 迭代器适配器（Adapters）—— 惰性转换
// 对标 C++ 的 std::transform / std::filter，但 RAII 更安全！
// 核心特征: 懒执行（Lazy），不调用消费者（如 collect）就不干活。
// ============================================================
fn adapters_demo() {
    println!("\n========== 3. 迭代器适配器（惰性） ==========");

    let v = vec![1, 2, 3, 4, 5];

    // 3.1 map() —— 转换（类似 std::transform）
    // 注意：这里只是创建了一个迭代器，还没有实际执行！
    let mapped = v.iter().map(|x| x * 2);
    println!("map 还没执行，只是创建了迭代器对象");

    // 必须用 collect() 或 for 循环来触发执行
    let doubled: Vec<i32> = mapped.collect();
    println!("map + collect: {:?}", doubled);

    // 3.2 filter() —— 过滤（类似 std::copy_if）
    let filtered: Vec<&i32> = v.iter().filter(|&&x| x > 3).collect();
    println!("filter (>3): {:?}", filtered);

    // 3.3 链式调用（map + filter 串联）
    let result: Vec<i32> = v
        .iter()
        .map(|x| x * 3)
        .filter(|&x| x % 2 == 0) // 保留偶数
        .collect();
    println!("map(*3) + filter(偶数): {:?}", result); // 3*2=6, 3*4=12 => [6, 12]

    // 3.4 take() —— 取前 N 个
    let taken: Vec<&i32> = v.iter().take(3).collect();
    println!("take(3): {:?}", taken);

    // 3.5 skip() —— 跳过前 N 个
    let skipped: Vec<&i32> = v.iter().skip(2).collect();
    println!("skip(2): {:?}", skipped);

    // 3.6 enumerate() —— 带索引（对应 C++ 的枚举）
    for (index, value) in v.iter().enumerate() {
        println!("v[{}] = {}", index, value);
    }

    // 3.7 zip() —— 合并两个迭代器（类似 Python 的 zip）
    let v1 = vec![1, 2, 3];
    let v2 = vec![10, 20, 30];
    let zipped: Vec<(i32, i32)> = v1.iter().zip(v2.iter()).map(|(&a, &b)| (a, b)).collect();
    println!("zip 合并: {:?}", zipped);

    // 3.8 rev() —— 反转
    let reversed: Vec<&i32> = v.iter().rev().collect();
    println!("rev 反转: {:?}", reversed);
}

// ============================================================
// 第四部分: 消费者（Consumers）—— 触发执行的终结操作
// 对标 C++ 的 accumulate / find / any_of
// ============================================================
fn consumers_demo() {
    println!("\n========== 4. 消费者（触发执行） ==========");

    let v = vec![1, 2, 3, 4, 5];

    // 4.1 collect() —— 收集到集合（最常用，已见多次）
    let collected: Vec<i32> = v.iter().map(|&x| x * 2).collect();
    println!("collect: {:?}", collected);

    // 4.2 sum() —— 求和
    let sum: i32 = v.iter().sum();
    println!("sum: {}", sum);

    // 4.3 product() —— 求乘积
    let product: i32 = v.iter().product();
    println!("product: {}", product);

    // 4.4 fold() —— 自定义归约（最强大的累积器，类似 C++ 的 accumulate）
    // 参数: 初始值(0), 闭包(累加器, 元素)
    let folded = v.iter().fold(0, |acc, &x| acc + x * 2);
    println!("fold (每个元素*2求和): {}", folded); // 2+4+6+8+10=30

    // 4.5 find() —— 查找第一个匹配的元素（返回 Option）
    let found = v.iter().find(|&&x| x == 3);
    println!("find 3: {:?}", found); // Some(3)

    let not_found = v.iter().find(|&&x| x == 99);
    println!("find 99: {:?}", not_found); // None

    // 4.6 any() / all() —— 是否存在 / 是否全部满足
    let has_even = v.iter().any(|&x| x % 2 == 0);
    println!("any 偶数: {}", has_even);

    let all_positive = v.iter().all(|&x| x > 0);
    println!("all > 0: {}", all_positive);

    // 4.7 max() / min() —— 最大最小值（返回 Option）
    let max = v.iter().max();
    println!("max: {:?}", max);
    let min = v.iter().min();
    println!("min: {:?}", min);

    // 4.8 count() —— 计数（会消耗迭代器）
    let count = v.iter().filter(|&&x| x > 2).count();
    println!(">2 的元素个数: {}", count);
}

// ============================================================
// 第五部分: 实战 —— 结合 Option 与 Result 的迭代器
// ============================================================
fn option_result_demo() {
    println!("\n========== 5. 迭代器 + Option/Result 实战 ==========");

    // 5.1 filter_map() —— 过滤并转换（同时处理 None）
    let nums = vec!["1", "2", "three", "4", "5"];
    let parsed: Vec<i32> = nums
        .iter()
        .filter_map(|s| s.parse::<i32>().ok()) // 成功解析的保留，失败的被过滤掉
        .collect();
    println!("filter_map 解析字符串: {:?}", parsed); // [1, 2, 4, 5]

    // 5.2 flatten() —— 展平嵌套迭代器
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<i32> = nested.iter().flatten().cloned().collect();
    println!("flatten 展平: {:?}", flat);

    // 5.3 实战：统计 HashMap 中大于指定值的元素
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 75);
    scores.insert("Charlie", 95);

    let threshold = 80;
    let high_scorers: Vec<&str> = scores
        .iter()
        .filter(|(_, &score)| score > threshold)
        .map(|(&name, _)| name)
        .collect();
    println!("分数 > {} 的人: {:?}", threshold, high_scorers);
}

// ============================================================
// 第六部分: 性能与安全性 —— 迭代器 vs 索引循环
// 结论: 迭代器更快且更安全（边界检查由迭代器内部处理，且无越界风险）
// ============================================================
fn performance_safety_demo() {
    println!("\n========== 6. 性能与安全性（击败 C++ 索引循环）==========");

    let v = vec![10, 20, 30, 40, 50];

    // 6.1 传统索引循环（有越界风险，需手动检查）
    let mut sum1 = 0;
    for i in 0..v.len() {
        sum1 += v[i]; // 如果 i 越界会 panic
    }
    println!("索引循环求和: {}", sum1);

    // 6.2 迭代器版本（更简洁，不可能越界）
    let sum2: i32 = v.iter().sum();
    println!("迭代器求和: {}", sum2);

    // 6.3 迭代器在编译时防止“迭代器失效”
    let mut vec = vec![1, 2, 3];
    // 下面的代码在 C++ 中是经典的未定义行为（修改容器时迭代器失效）
    // 在 Rust 中，编译器直接拒绝编译！
    // for item in &vec {
    //     vec.push(4); // ❌ 编译错误！不能在有不可变借用时修改 vec
    // }
    // 正确做法：如果想在迭代时修改，用索引或收集新数据。
    let mut vec2 = vec![1, 2, 3];
    // 方式1：收集新值再赋值
    let new_vec: Vec<i32> = vec2.iter().map(|&x| x * 2).collect();
    vec2 = new_vec;
    println!("安全地迭代修改: {:?}", vec2);

    // 方式2：用索引循环修改（需要手动管理）
    let mut vec3 = vec![1, 2, 3];
    for i in 0..vec3.len() {
        vec3[i] *= 2;
    }
    println!("索引循环修改: {:?}", vec3);
}

// ============================================================
// 第七部分: 综合实战 —— 文件数据处理
// ============================================================
fn real_world_demo() {
    println!("\n========== 7. 综合实战: 处理日志数据 ==========");

    // 模拟日志数据（实际中从文件读取）
    let logs = vec![
        "INFO 2026-07-23 User login",
        "ERROR 2026-07-23 Database timeout",
        "INFO 2026-07-23 File uploaded",
        "WARN 2026-07-23 High memory usage",
        "ERROR 2026-07-23 Connection reset",
    ];

    // 需求：提取所有 ERROR 级别的日志，并转为大写描述
    let error_logs: Vec<String> = logs
        .iter()
        .filter(|line| line.starts_with("ERROR"))
        .map(|line| {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                format!("🚨 [{}] {}", parts[1], parts[2].to_uppercase())
            } else {
                line.to_string()
            }
        })
        .collect();

    println!("错误日志（格式化后）:");
    for log in &error_logs {
        println!("  {}", log);
    }

    // 统计每种日志级别的数量
    use std::collections::HashMap;
    let mut level_count = HashMap::new();
    for line in &logs {
        let level = line.split_whitespace().next().unwrap_or("UNKNOWN");
        *level_count.entry(level).or_insert(0) += 1;
    }
    println!("日志级别统计: {:?}", level_count);
}

// ============================================================
// 主函数入口
// ============================================================
fn main() {
    println!("🔥 Rust 迭代器完全指南");
    println!("（请对照 C++ 的 std::transform / std::accumulate 理解）\n");

    iterator_types_demo();
    iterator_trait_demo();
    adapters_demo();
    consumers_demo();
    option_result_demo();
    performance_safety_demo();
    real_world_demo();

    println!("\n✅ 所有迭代器演示完成！");
    println!("💡 记住黄金法则:");
    println!("  1. iter() 只读，iter_mut() 修改，into_iter() 消费");
    println!("  2. 适配器（map/filter）是惰性的，必须加 collect() 或 for 才会执行");
    println!("  3. 迭代器比索引循环更安全、更快（编译时检查）");
    println!("  4. 链式调用让代码像流水线一样清晰！");
}