// ============================================================
// 文件名: main.rs
// 功能: Rust 数组（[T; N]）完整教学 —— 对标 C++ 的 std::array
// 涵盖: 创建、初始化、访问、遍历、输入输出、切片、转换
// ============================================================

use std::io::{self, BufRead};

// ============================================================
// 第一部分: 数组的创建与初始化
// ============================================================
fn array_creation_demo() {
    println!("\n========== 1. 创建与初始化 ==========");

    // 1.1 直接指定元素（类型自动推断为 [i32; 5]）
    let arr1 = [1, 2, 3, 4, 5];
    println!("arr1 = {:?}", arr1);

    // 1.2 显式类型标注
    let arr2: [f64; 3] = [1.0, 2.0, 3.0];
    println!("arr2 = {:?}", arr2);

    // 1.3 用相同值填充（类似 C++ 的 std::array<int, 5>{} 但可指定初始值）
    let arr3 = [0; 10]; // 10 个 0
    println!("arr3 = {:?}", arr3);

    // 1.4 从变量初始化（必须类型一致）
    let a = 10;
    let b = 20;
    let arr4 = [a, b, a + b];
    println!("arr4 = {:?}", arr4);

    // 1.5 数组长度是类型的一部分，不同长度是不同类型
    let arr5: [i32; 3] = [1, 2, 3];
    // let arr6: [i32; 4] = arr5; // ❌ 编译错误：类型不匹配
}

// ============================================================
// 第二部分: 访问与修改元素（安全与不安全）
// ============================================================
fn array_access_demo() {
    println!("\n========== 2. 访问与修改 ==========");

    let mut arr = [10, 20, 30, 40, 50];

    // 2.1 索引访问（越界会 panic 导致程序崩溃，类似 C++ vector::at()）
    let first = arr[0];
    println!("arr[0] = {}", first);
    arr[2] = 99; // 修改
    println!("修改后 arr = {:?}", arr);

    // 2.2 安全访问：使用 get() 返回 Option<&T>（不会 panic）
    match arr.get(5) {
        Some(val) => println!("arr[5] = {}", val),
        None => println!("索引 5 越界，安全返回 None"),
    }

    // 2.3 获取数组长度
    println!("数组长度: {}", arr.len());

    // 2.4 获取数组占用的字节数
    println!("数组占用 {} 字节", std::mem::size_of_val(&arr));
}

// ============================================================
// 第三部分: 遍历数组
// ============================================================
fn array_iteration_demo() {
    println!("\n========== 3. 遍历 ==========");

    let arr = [1, 2, 3, 4, 5];

    // 3.1 不可变引用遍历（推荐，不移动所有权）
    print!("for &item in &arr: ");
    for item in &arr {
        print!("{} ", item);
    }
    println!();

    // 3.2 可变引用遍历（修改元素）
    let mut arr_mut = [1, 2, 3];
    for item in &mut arr_mut {
        *item *= 2;
    }
    println!("修改后: {:?}", arr_mut);

    // 3.3 按值遍历（移动所有权，仅对 Copy 类型安全）
    // 对 i32 等 Copy 类型，迭代时自动复制，所有权不受影响
    let arr_copy = [10, 20, 30];
    for item in arr_copy {
        print!("{} ", item);
    }
    println!();
    // 之后 arr_copy 仍然可用（因为 i32 是 Copy）
    println!("arr_copy 仍有效: {:?}", arr_copy);

    // 3.4 获取索引和值（enumerate）
    for (i, val) in arr.iter().enumerate() {
        println!("arr[{}] = {}", i, val);
    }

    // 3.5 使用迭代器适配器（例如 map、filter）
    let doubled: Vec<_> = arr.iter().map(|x| x * 2).collect();
    println!("每个元素乘以2: {:?}", doubled);
}

// ============================================================
// 第四部分: 数组的输入（从 stdin 读取填充）
// ============================================================
fn array_input_demo() {
    println!("\n========== 4. 数组输入 ==========");
    println!("请输入 5 个整数，用空格分隔（例如: 1 2 3 4 5）:");

    // 读取一行
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");

    // 按空白字符分割，并解析为 i32
    let numbers: Result<Vec<i32>, _> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect();

    match numbers {
        Ok(vec) => {
            if vec.len() == 5 {
                // 从 Vec 转为数组（需要 TryInto）
                // 方式1: 使用 try_into()（推荐，类型安全）
                let arr: [i32; 5] = vec.try_into().expect("长度必须为5");
                println!("输入的数组: {:?}", arr);

                // 方式2: 逐个赋值（如果长度不匹配则 panic）
                // let mut arr = [0; 5];
                // arr.copy_from_slice(&vec); // 需要 vec 长度 >= 5
            } else {
                println!("输入了 {} 个数字，但期望 5 个", vec.len());
            }
        }
        Err(e) => println!("解析错误: {:?}", e),
    }
}

// ============================================================
// 第五部分: 数组的输出（多种格式化方式）
// ============================================================
fn array_output_demo() {
    println!("\n========== 5. 数组输出 ==========");

    let arr = [1, 2, 3, 4, 5];

    // 5.1 使用调试格式 {:?}（最常用）
    println!("Debug 格式: {:?}", arr);

    // 5.2 使用漂亮格式 {:#?}（带换行和缩进）
    println!("漂亮格式: {:#?}", arr);

    // 5.3 自定义格式化（用循环拼接）
    print!("自定义输出: [");
    for (i, val) in arr.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", val);
    }
    println!("]");

    // 5.4 将数组转为字符串（用 join，但仅适用于可显示类型）
    // 注意：join 用于字符串切片，如果元素不是 &str 需要先转换
    let str_arr: Vec<String> = arr.iter().map(|&x| x.to_string()).collect();
    let joined = str_arr.join(", ");
    println!("用 join 连接: {}", joined);
}

// ============================================================
// 第六部分: 数组切片（Slices）—— 借用子数组
// ============================================================
fn array_slice_demo() {
    println!("\n========== 6. 数组切片 ==========");

    let arr = [10, 20, 30, 40, 50];

    // 切片是 &[T]，是对数组部分的引用
    let slice1: &[i32] = &arr[1..4]; // [20, 30, 40]
    println!("arr[1..4] = {:?}", slice1);

    // 全切片
    let slice2 = &arr[..]; // 全部
    println!("全切片: {:?}", slice2);

    // 切片可以用于函数参数（更通用，因为可以接受数组或 Vec）
    fn sum_slice(s: &[i32]) -> i32 {
        s.iter().sum()
    }
    println!("slice1 的和: {}", sum_slice(slice1));
    println!("arr 作为切片传入的和: {}", sum_slice(&arr));

    // 从切片生成数组（需要长度匹配）
    let arr2: [i32; 3] = slice1.try_into().expect("长度应为3");
    println!("从切片复制的数组: {:?}", arr2);
}

// ============================================================
// 第七部分: 数组与 Vec 的互转
// ============================================================
fn array_vec_conversion_demo() {
    println!("\n========== 7. 数组 ↔ Vec 转换 ==========");

    let arr = [1, 2, 3, 4, 5];

    // 7.1 数组 -> Vec（通过 to_vec 或 iter().collect()）
    let vec1 = arr.to_vec();
    let vec2: Vec<_> = arr.iter().cloned().collect();
    println!("从数组得到 Vec: {:?}, {:?}", vec1, vec2);

    // 7.2 Vec -> 数组（需确认长度）
    let vec = vec![10, 20, 30];
    let arr_from_vec: [i32; 3] = vec.try_into().expect("长度必须为3");
    println!("从 Vec 转为数组: {:?}", arr_from_vec);

    // 如果长度不匹配，可以手动转换（例如取前 N 个）
    let vec_long = vec![1, 2, 3, 4, 5, 6];
    let arr_from_part: [i32; 3] = vec_long[0..3].try_into().unwrap();
    println!("取前3个转为数组: {:?}", arr_from_part);
}

// ============================================================
// 第八部分: 多维数组（数组的数组）
// ============================================================
fn multi_dimensional_demo() {
    println!("\n========== 8. 多维数组 ==========");

    // 2x3 的二维数组（类型为 [[i32; 3]; 2]）
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];

    println!("二维数组: {:?}", matrix);
    println!("matrix[0][1] = {}", matrix[0][1]);

    // 遍历二维数组
    for row in &matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }

    // 三维数组（3x2x3）
    let cube = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
    println!("三维数组: {:?}", cube);
}

// ============================================================
// 第九部分: 综合实战 —— 从用户输入构造数组并计算统计值
// ============================================================
fn combined_practice() {
    println!("\n========== 9. 综合实战: 输入数组并计算 ==========");

    println!("请输入任意数量的整数，以空格分隔（例如: 1 2 3 4 5）:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取失败");

    // 解析为 Vec<i32>
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if numbers.is_empty() {
        println!("没有输入有效数字");
        return;
    }

    // 计算和、平均值、最大最小值
    let sum: i32 = numbers.iter().sum();
    let len = numbers.len();
    let avg = sum as f64 / len as f64;
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();

    println!("输入的数字: {:?}", numbers);
    println!("数量: {}", len);
    println!("总和: {}", sum);
    println!("平均值: {:.2}", avg);
    println!("最大值: {}", max);
    println!("最小值: {}", min);

    // 如果能转为固定长度数组（假设需要 5 个），演示转换
    if len == 5 {
        let arr: [i32; 5] = numbers.try_into().expect("长度应为5");
        println!("转为固定数组: {:?}", arr);
    }
}

// ============================================================
// 主函数入口
// ============================================================
fn main() {
    println!("🔥 Rust 数组（[T; N]）完全指南");
    println!("（请对照 C++ 的 std::array 或原生数组来理解）\n");

    array_creation_demo();
    array_access_demo();
    array_iteration_demo();

    // 注意：array_input_demo 需要用户交互，如果想避免多次输入，可注释掉
    // 但为了演示，保留它，你可以选择运行一次输入。
    array_input_demo();

    array_output_demo();
    array_slice_demo();
    array_vec_conversion_demo();
    multi_dimensional_demo();

    // 综合实战，需要额外输入，也可注释
    combined_practice();

    println!("\n✅ 所有数组演示完成！");
    println!("💡 记住关键点:");
    println!("  1. 数组固定大小，是值类型（分配在栈上）");
    println!("  2. 数组长度是类型的一部分（[i32; 5] 和 [i32; 6] 不同）");
    println!("  3. 访问越界会 panic，用 get() 安全访问");
    println!("  4. 使用切片 &[T] 来传递数组的部分或全部（零拷贝）");
    println!("  5. 如需动态大小，请使用 Vec<T>");
}