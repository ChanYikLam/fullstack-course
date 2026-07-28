// ============================================================
// 文件名: main.rs
// 功能: Rust 三大核心数据结构 —— String/&str, Vec, HashMap
//       完整教学演示，涵盖创建、修改、遍历、所有权、转换等
// 作者: 教学助手
// ============================================================

use std::collections::HashMap;

// ============================================================
// 第一部分: String 与 &str —— 所有权与借用的精髓
// ============================================================
fn string_demo() {
    println!("\n========== String 与 &str 演示 ==========");

    // ----- 1. 创建字符串 -----
    let static_str: &str = "Hello, world!"; // 字面量，'static
    let mut owned_string = String::from("Rust"); // 从 &str 创建
    let another = "Rust".to_string(); // 同样
    let formatted = format!("{} is {}", "Rust", "great"); // 格式化

    println!("静态字符串: {}", static_str);
    println!("拥有的 String: {}", owned_string);
    println!("另一个 String: {}", another);
    println!("格式化结果: {}", formatted);

    // ----- 2. 修改 String（可变）-----
    owned_string.push('!'); // 追加字符
    owned_string.push_str(" 2026"); // 追加字符串
    println!("修改后: {}", owned_string); // "Rust! 2026"

    // ----- 3. 转换: String ↔ &str -----
    // String -> &str（借用）
    let borrowed: &str = &owned_string; // 自动解引用强制转换
    let borrowed_explicit = owned_string.as_str();
    // &str -> String（拷贝）
    let new_string = borrowed.to_string();

    println!("借用 &str: {}", borrowed);
    println!("新 String: {}", new_string);

    // ----- 4. 字符串拼接（注意所有权转移）-----
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2; // s1 被移动（所有权转移），s2 被借用
    println!("拼接结果: {}", s3);
    // println!("{}", s1); // ❌ 编译错误：s1 已失效

    // 若想保留 s1，用 clone 或 format!
    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = format!("{} {}", s4, s5); // 不转移所有权
    println!("format 拼接: {} (s4和s5仍可用)", s6);
    println!("s4 = {}, s5 = {}", s4, s5);

    // ----- 5. 字符串切片（字节边界需谨慎）-----
    let chinese = "你好世界";
    // 切片按字节索引，必须落在字符边界，否则 panic
    let slice = &chinese[0..3]; // "你"（每个中文字符占 3 字节）
    println!("中文切片: {}", slice);
    // 安全遍历字符（推荐）
    print!("字符迭代: ");
    for ch in chinese.chars() {
        print!("{} ", ch);
    }
    println!();

    // ----- 6. 函数参数接受 &str 更通用 -----
    fn greet(name: &str) {
        println!("你好，{}!", name);
    }
    greet("Bob"); // 字面量
    greet(&owned_string); // &String 自动转为 &str
}

// ============================================================
// 第二部分: Vec<T> —— 动态数组，对标 std::vector
// ============================================================
fn vec_demo() {
    println!("\n========== Vec 动态数组演示 ==========");

    // ----- 1. 创建 -----
    let v1: Vec<i32> = Vec::new(); // 空向量
    let v2 = vec![1, 2, 3]; // 宏初始化
    let v3 = vec![0; 5]; // 长度为 5，全 0
    println!("v2 = {:?}", v2);
    println!("v3 = {:?}", v3);

    // ----- 2. 添加与删除 -----
    let mut v = vec![10, 20];
    v.push(30);
    v.push(40);
    println!("push 后: {:?}", v);
    if let Some(last) = v.pop() {
        println!("弹出最后一个: {}", last);
    }
    println!("pop 后: {:?}", v);

    // ----- 3. 访问元素（安全 vs 不安全）-----
    // 索引（越界会 panic）
    let val = v[1]; // 复制（i32 是 Copy）
    println!("v[1] = {}", val);
    // get 返回 Option（安全）
    match v.get(10) {
        Some(x) => println!("找到了: {}", x),
        None => println!("索引 10 越界"),
    }

    // ----- 4. 遍历 -----
    // 不可变借用遍历
    print!("遍历 &v: ");
    for x in &v {
        print!("{} ", x);
    }
    println!();

    // 可变遍历（修改元素）
    for x in &mut v {
        *x *= 2;
    }
    println!("乘以2后: {:?}", v);

    // 消费遍历（移动所有权），之后 v 失效
    let v_move = vec!["a".to_string(), "b".to_string()];
    for s in v_move {
        println!("消费: {}", s);
    }
    // println!("{:?}", v_move); // ❌ 编译错误

    // ----- 5. 切片（借用子数组）-----
    let v_slice = &v[1..3]; // 引用范围
    println!("切片 [1..3] = {:?}", v_slice);

    // ----- 6. 容量与预留空间 -----
    let mut v_cap = Vec::with_capacity(10);
    println!("初始容量: {}, 长度: {}", v_cap.capacity(), v_cap.len());
    for i in 0..15 {
        v_cap.push(i);
    }
    println!("添加15个元素后容量: {}, 长度: {}", v_cap.capacity(), v_cap.len());
}

// ============================================================
// 第三部分: HashMap<K, V> —— 哈希表，对标 std::unordered_map
// ============================================================
fn hashmap_demo() {
    println!("\n========== HashMap 哈希表演示 ==========");

    // ----- 1. 创建与插入 -----
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("初始 map: {:?}", scores);

    // ----- 2. 取值（安全）-----
    match scores.get("Blue") {
        Some(score) => println!("Blue 的分数: {}", score),
        None => println!("不存在"),
    }

    // ----- 3. 更新（插入或覆盖）-----
    scores.insert(String::from("Blue"), 25); // 覆盖
    println!("覆盖后: {:?}", scores);

    // ----- 4. 使用 Entry API（高效）-----
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", word_count);

    // ----- 5. 从迭代器构建 -----
    let keys = vec![String::from("a"), String::from("b")];
    let values = vec![1, 2];
    let map: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();
    println!("从迭代器构建: {:?}", map);

    // ----- 6. 遍历 -----
    println!("遍历所有键值对:");
    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    }

    // ----- 7. 所有权规则（非 Copy 类型会移动）-----
    let key = String::from("Green");
    let value = 30;
    scores.insert(key, value); // key 被移动
    // println!("{}", key); // ❌ 编译错误，key 已失效
    println!("插入 Green 后: {:?}", scores);
}

// ============================================================
// 第四部分: 综合实战 —— 结合三种数据结构处理文本
// ============================================================
fn combined_demo() {
    println!("\n========== 综合实战: 统计单词长度 ==========");

    let text = String::from("Rust is a systems programming language that runs blazingly fast");
    // 分割成单词，得到 Vec<&str>（借用自 text）
    let words: Vec<&str> = text.split_whitespace().collect();

    // 建立 HashMap: 单词 -> 长度
    let mut word_len_map = HashMap::new();
    for word in &words {
        // 如果单词已存在，不覆盖（or_insert 保持原值）
        word_len_map.entry(*word).or_insert(word.len());
    }

    // 按字母顺序输出
    let mut sorted_words: Vec<&str> = word_len_map.keys().cloned().collect();
    sorted_words.sort();
    println!("单词长度统计（按字母序）:");
    for w in sorted_words {
        if let Some(&len) = word_len_map.get(w) {
            println!("  {} -> {}", w, len);
        }
    }

    // 额外演示：通过 Vec 收集长度并求平均
    let total_len: usize = words.iter().map(|w| w.len()).sum();
    let avg = total_len as f64 / words.len() as f64;
    println!("平均单词长度: {:.2}", avg);
}

// ============================================================
// 主函数入口
// ============================================================
fn main() {
    string_demo();
    vec_demo();
    hashmap_demo();
    combined_demo();
    println!("\n✅ 所有演示完成！");
}