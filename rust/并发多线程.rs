// ============================================================
// 文件名: main.rs
// 功能: Rust 并发（Concurrency）完全教学
// 对标 C++: std::thread, std::mutex, std::shared_ptr, 条件变量
// 涵盖: 线程生成、move 语义、通道、Arc<Mutex>、Send/Sync
// ============================================================

use std::thread;
use std::time::Duration;
use std::sync::mpsc; // 多生产者单消费者（Multi-producer, single-consumer）
use std::sync::{Arc, Mutex};
use std::rc::Rc; // 仅用于演示编译错误

// ============================================================
// 第一部分: 基础线程（spawn + join）
// 对标 C++: std::thread t(func); t.join();
// ============================================================
fn basic_thread_demo() {
    println!("\n========== 1. 基础线程（spawn + join）==========");

    // 创建一个线程，打印信息
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("子线程: 计数 {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程也做一些工作
    for i in 1..3 {
        println!("主线程: 计数 {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    // 等待子线程结束（必须 join，否则主线程退出时子线程会被强制终止）
    handle.join().unwrap();
    println!("子线程已结束");
}

// ============================================================
// 第二部分: move 闭包 —— 转移所有权到线程（最重要的安全机制）
// 对标 C++: 按值捕获 [=] 或 std::move 到 lambda
// 核心: 防止主线程释放数据后子线程还在使用（悬垂引用）
// ============================================================
fn move_closure_demo() {
    println!("\n========== 2. move 闭包（所有权转移）==========");

    let data = vec![1, 2, 3, 4, 5];

    // 如果不加 move，下面的代码会编译失败：
    // let handle = thread::spawn(|| {
    //     println!("子线程: {:?}", data); // ❌ data 的生命周期可能短于线程
    // });

    // 正确做法：使用 move 将 data 的所有权转移到子线程
    let handle = thread::spawn(move || {
        println!("子线程拥有数据: {:?}", data);
        // data 在这里被释放（子线程结束时）
    });

    // println!("主线程: {:?}", data); // ❌ 编译错误！data 已被移动
    handle.join().unwrap();

    // 对于 Copy 类型（如 i32），move 会拷贝而非移动
    let num = 42;
    let handle2 = thread::spawn(move || {
        println!("子线程中的数字: {}", num);
    });
    handle2.join().unwrap();
    println!("主线程仍然可以使用 num（因为 i32 是 Copy）: {}", num); // ✅ 有效
}

// ============================================================
// 第三部分: 消息传递 —— mpsc::channel（Rust 的王牌武器）
// 对标 C++: std::queue + std::mutex + std::condition_variable
// 优势: 无需手动加锁，不会出现虚假唤醒，类型安全！
// ============================================================
fn channel_demo() {
    println!("\n========== 3. 消息通道（mpsc::channel）==========");

    // 创建通道（发送端 tx，接收端 rx）
    let (tx, rx) = mpsc::channel();

    // 子线程：发送消息
    let tx_clone = tx.clone(); // 克隆发送端（多生产者需要）
    let handle1 = thread::spawn(move || {
        let messages = vec!["Hello", "from", "Rust", "thread"];
        for msg in messages {
            tx_clone.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程：接收消息（迭代器方式，阻塞直到通道关闭）
    println!("接收到的消息:");
    for received in rx {
        println!("  {}", received);
    }
    // 注意：这里 rx 迭代会在所有 tx 被 drop 后自动结束

    handle1.join().unwrap();

    // 演示多个生产者（克隆 tx）
    let (tx2, rx2) = mpsc::channel();
    let tx_a = tx2.clone();
    let tx_b = tx2.clone();

    let h1 = thread::spawn(move || {
        tx_a.send("来自线程 A").unwrap();
    });
    let h2 = thread::spawn(move || {
        tx_b.send("来自线程 B").unwrap();
    });

    // 主线程接收两个消息
    for _ in 0..2 {
        let msg = rx2.recv().unwrap();
        println!("收到: {}", msg);
    }
    h1.join().unwrap();
    h2.join().unwrap();

    // 核心对比 C++：
    // C++ 需要你手动管理队列、互斥锁、条件变量（且极易出错）。
    // Rust 的 channel 在编译时保证：发送的数据类型必须一致，接收端自动阻塞。
}

// ============================================================
// 第四部分: 共享状态 —— Arc<Mutex<T>>（原子引用计数 + 互斥锁）
// 对标 C++: std::shared_ptr<std::mutex<T>> 
// 核心: 多线程共享并修改同一份数据
// ============================================================
fn shared_state_demo() {
    println!("\n========== 4. 共享状态（Arc<Mutex<T>>）==========");

    // 错误的尝试：使用 Rc（不是线程安全的）
    // let counter = Rc::new(Mutex::new(0));
    // let counter_clone = Rc::clone(&counter);
    // thread::spawn(move || {
    //     *counter_clone.lock().unwrap() += 1;
    // }); // ❌ 编译错误：Rc 不能在线程间传递（没有实现 Send）

    // 正确做法：使用 Arc（原子引用计数，线程安全）
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 启动 10 个线程，每个线程都增加计数器
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 锁定互斥量（守卫），自动解锁（RAII）
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            // 守卫在这里离开作用域，自动释放锁
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 最终结果应该是 10
    println!("最终计数器值: {}", *counter.lock().unwrap());

    // ---- 高级技巧：Mutex 的优雅使用 ----
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    let mut thread_handles = vec![];

    for i in 0..5 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            vec.push(i); // 每个线程向 Vec 中 push 一个数
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    // 打印最终 Vec（顺序不确定，因为是并发操作）
    println!("共享 Vec 的内容: {:?}", *shared_data.lock().unwrap());

    // 注意：Mutex 中毒（Poisoning）
    // 如果持有锁的线程 panic，锁会“中毒”。后续线程获取锁会返回 Err。
    // 这比 C++ 的未定义行为安全得多！
}

// ============================================================
// 第五部分: Send 与 Sync —— 并发安全的核心 Trait（幕后原理）
// 对标 C++: std::is_trivially_copyable / 线程安全类型标记（但 Rust 是语言内置的）
// 规则: 
//   - Send: 可以安全地将所有权转移到另一个线程（绝大多数类型都是 Send）
//   - Sync: 可以在多个线程间安全地共享引用（&T 是 Send 时，T 是 Sync）
// 注意: Rc<T> 既不是 Send 也不是 Sync，所以跨线程用 Arc<T>。
// ============================================================
fn send_sync_demo() {
    println!("\n========== 5. Send 与 Sync 原理 ==========");

    // 5.1 检查类型是否实现了 Send / Sync
    // 标准库提供的类型通常都实现了
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    // i32 既是 Send 也是 Sync
    is_send::<i32>();
    is_sync::<i32>();

    // String 是 Send 但不是 Sync（因为可变性，不能安全地共享引用）
    is_send::<String>();
    // is_sync::<String>(); // ❌ 编译错误

    // Arc<i32> 既是 Send 也是 Sync
    is_send::<Arc<i32>>();
    is_sync::<Arc<i32>>();

    // Rc<i32> 既不是 Send 也不是 Sync
    // is_send::<Rc<i32>>(); // ❌
    // is_sync::<Rc<i32>>(); // ❌

    // 5.2 手动实现 Send / Sync 是 unsafe 的（除非你非常清楚自己在做什么）
    // 99.9% 的情况下，你只需要使用标准库提供的组合（Arc, Mutex 等）

    // 5.3 实战演示：为什么 Rc 不能跨线程
    // 下面这段代码如果解开注释会编译失败：
    // let rc = Rc::new(42);
    // thread::spawn(move || {
    //     println!("{}", rc);
    // }).join().unwrap();

    // 而 Arc 可以：
    let arc = Arc::new(42);
    let handle = thread::spawn(move || {
        println!("Arc 跨线程: {}", arc);
    });
    handle.join().unwrap();

    println!("Send/Sync 保证了跨线程内存安全！");
}

// ============================================================
// 第六部分: 实战 —— 并发计算器（多线程累加）
// ============================================================
fn real_world_demo() {
    println!("\n========== 6. 实战: 并发计算器 ==========");

    // 任务：将一个大型数组分段交给多个线程求和，最后合并结果
    let data: Vec<i32> = (1..=1000).collect(); // 1 到 1000
    let num_threads = 8;
    let chunk_size = data.len() / num_threads;

    // 使用 Arc 共享数据（但这里我们只读，所以用 Arc<Vec<i32>> 也是安全的）
    let shared_data = Arc::new(data);

    let mut handles = vec![];

    for i in 0..num_threads {
        let data_clone = Arc::clone(&shared_data);
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            data_clone.len() // 最后一个线程取剩余所有
        } else {
            start + chunk_size
        };

        let handle = thread::spawn(move || {
            // 切片（只读）
            let slice = &data_clone[start..end];
            let sum: i32 = slice.iter().sum();
            println!("线程 {} 计算了 [{}-{}] 的和 = {}", i, start, end, sum);
            sum
        });
        handles.push(handle);
    }

    // 汇总所有线程的结果
    let mut total = 0;
    for handle in handles {
        let partial_sum = handle.join().unwrap();
        total += partial_sum;
    }

    println!("总结果: {}", total);
    // 验证：1 到 1000 的和 = 1000 * 1001 / 2 = 500500
    println!("验证: 500500");
}

// ============================================================
// 第七部分: 高级 —— 通道 + 多生产者（工作池模式）
// 模拟一个简易的任务队列（类似线程池）
// ============================================================
fn worker_pool_demo() {
    println!("\n========== 7. 实战: 简易工作池（通道 + 多线程）==========");

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx)); // 多个线程需要共享接收端

    // 启动 4 个工作线程
    let mut worker_handles = vec![];
    for id in 0..4 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                // 获取任务（锁住接收端）
                let job = {
                    let rx_guard = rx_clone.lock().unwrap();
                    // recv() 会阻塞直到有任务
                    match rx_guard.recv() {
                        Ok(job) => job,
                        Err(_) => {
                            // 发送端被关闭，退出循环
                            println!("工作线程 {} 检测到通道关闭，退出", id);
                            break;
                        }
                    }
                };
                // 执行任务
                println!("工作线程 {} 执行任务: {}", id, job);
                thread::sleep(Duration::from_millis(50));
            }
        });
        worker_handles.push(handle);
    }

    // 主线程发送 10 个任务
    let sender = tx;
    for i in 1..=10 {
        sender.send(format!("任务 #{}", i)).unwrap();
        println!("主线程发送了任务 #{}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // 关闭通道（drop 发送端）
    drop(sender); // 让所有工作线程退出

    // 等待所有工作线程完成
    for handle in worker_handles {
        handle.join().unwrap();
    }
    println!("所有任务已完成！");
}

// ============================================================
// 主函数入口
// ============================================================
fn main() {
    println!("🔥 Rust 并发（Concurrency）完全指南");
    println!("（请对照 C++ 的 std::thread / std::mutex / std::async 理解）\n");

    basic_thread_demo();
    move_closure_demo();
    channel_demo();
    shared_state_demo();
    send_sync_demo();
    real_world_demo();
    worker_pool_demo();

    println!("\n✅ 所有并发演示完成！");
    println!("💡 记住黄金法则:");
    println!("  1. 线程之间传递数据必须用 `move` 闭包（转移所有权）");
    println!("  2. `mpsc::channel` 比手动加锁安全得多（优先使用）");
    println!("  3. 多线程共享修改数据: `Arc<Mutex<T>>`");
    println!("  4. 多线程只读共享: `Arc<T>` 就够了");
    println!("  5. `Send` 和 `Sync` 是自动推导的——相信编译器！");
    println!("  6. 死锁不可避免（运行时），但 Mutex 中毒机制让你知道出了问题！");
}