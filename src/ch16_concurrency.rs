use core::borrow;
use std::rc::Rc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::cell::{Cell, RefCell};

/*
Send：如果一个类型实现了 Send，意味着这个类型的值可以安全地在线程间转移所有权
Sync：如果一个类型实现了 Sync，意味着这个类型的不可变引用可以安全地在多个线程间共享

In other words, any type T implements Sync if &T (an immutable reference to T ) implements Send
    因为不可变引用可以复制，如果 &T 可以安全地 Send 到一个线程，就意味着可以 Send
    到多个线程，从而实现多线程共享，这就是 Sync 的定义。
*/

fn _move_refType() {
    let x = 42;
    // 因为 i32: Sync，所以 &i32 可以 Send 到其他线程
    // 若没有move,则 closure may outlive the current function, but it borrows `x`, which is owned by the current function
    thread::spawn(move || {
        let reference: &i32 = &x; // &i32 可以跨线程
        println!("--- {}", reference);
    });
    /*
    - Copy 类型（如 i32, u32, bool, char, f64 等）：
        move 时会复制，原变量仍然可用
        包括所有基本数据类型和纯粹由 Copy 类型组成的结构体
    - 非 Copy 类型（如 String, Vec, Box 等）：
        move 时会转移所有权，原变量不再可用
        包括所有需要堆分配或包含非 Copy 字段的类型 */
    println!("=== {}", x);

    // 引用的 Copy 行为
    {
        let x = 42;
        let r: &i32 = &x;  // &i32 实现了 Copy trait

        // 共享引用会被复制，不是移动
        let r2 = r;  // r 被复制给 r2（因为 &i32: Copy）
        println!("r={}", r);   // ✅ r 仍然可用（被复制了）
        println!("r2={}", r2);

        let s = String::from("hello"); // String: !Copy
        let sr1 = &s;
        let sr2 = sr1;
        println!("sr1={}", sr1); // ✅ sr1 仍然可用，共享引用会被复制

        // 对比：可变引用不能 Copy
        let mut y = 42;
        let mr = &mut y;       // &mut i32 没有实现 Copy
        let mr2 = mr;          // mr 被 move 给 mr2
        // println!("{}", mr); // ❌ mr 已被 move，不能再使用
        println!("mr2={}", mr2); // ✅ 只有 mr2 可用
    }
}

fn t1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("num:{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("num:{} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 主线程结束终止新线程

    _move_refType();

    let cell = Cell::new(42);
    // ✅ 这样可以：move 转移所有权（Cell支持 Send）
    thread::spawn(move || {
        println!("print cell: {}", cell.get());
    });

    // ❌ 这样会报错：共享引用（需要 Sync）
    let cell2 = Cell::new(42);
    // thread::spawn(|| {
    //     println!("print cell: {}", cell2.get()); // 错误：Cell<i32>` cannot be shared between threads safely
    // });

    // ❌ 即使用 Arc 包装也不行，因为 Cell: !Sync
    let cell3 = Arc::new(Cell::new(42));
    // let cell3_clone = Arc::clone(&cell3);
    // thread::spawn(move || {
    //     println!("print cell: {}", cell3_clone.get()); // 错误：Cell<i32>` cannot be shared between threads safely
    // });

    // Rc<T> 不是 Send
    let rc = Rc::new(42);
    // Rc: 不支持 Send
    // thread::spawn(move || {
    //     println!("{}", rc); // Rc<i32>` cannot be sent between threads safely
    // });

    let cell_str = Cell::new(String::from("hello"));
    let old_str = cell_str.replace(String::from("world"));
    // error: the following trait bounds were not satisfied: `std::string::String: std::marker::Copy
    //let ss = cell_str.get();

    let refcell = RefCell::new(42);
    let borrow1 = refcell.borrow();
    let borrow2 = refcell.borrow();
    // at runtime: already borrowed: BorrowMutError
    // let mut borrow_mut = refcell.borrow_mut();
    //*borrow_mut += 1; // when no `mut` given, compile time report: cannot borrow `borrow_mut` as mutable, as it is not declared as mutable
}

fn t_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("num:{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("num:{} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // Because unwrap may panic, its use is generally discouraged. Panics are meant for unrecoverable errors
    handle.join().unwrap();
}

// P506
fn t_borrow_by_move() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("here's a vec:{:?}", v);
    });

    // closure may outlive the current function, but it borrows `v2`, which is owned by the current function
    // let v2 = vec![1,2,3];
    // let handle = thread::spawn(|| {
    //     println!("here's a vec:{:?}", v2);
    // });

    handle.join().unwrap();
}

fn t_use_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        // send会返回 Result<T,E>，当接收端无法继续时，执行发送操作就会返回一个错误，这里
        // 直接调用unwrap来触发panic，实际中应妥善处理错误
        // never block the current thread; 会获取参数的所有权，并在传递时将所有权转给接收者
        tx.send(val).unwrap();
        // println!("val is {}", val);
    });
    // if sender close, then recv report error
    // try_recv will not block
    let received = rx.recv().unwrap();
    println!("got:{}", received);

    let (tx2, rx2) = mpsc::channel();
    for i in 0..3 {
        let tx2_clone = tx2.clone();
        thread::spawn(move || {
            tx2_clone.send(i).unwrap();
        });
    }

    // 需要丢弃原始的发送端,如果不做，recv接收完发送的后，会卡住
    // 因为只是移动了副本到线程中，还剩下一个tx 在主线程中
    drop(tx2);
    for received in rx2 {
        println!("Got: {}", received);
    }
}

fn t_send_multiple_to_receiver() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
        // tx 被完全移动到线程中，线程结束时自动 drop，没有遗留的发送端
    });

    // 将rx视为迭代器，不再显式调用recv. 并在通道关闭时退出循环
    for received in rx {
        println!("got:{}", received);
    }
}

fn t_use_multiple_sender() {
    let (tx, rx) = mpsc::channel();
    // 首先本身是mp，所以允许多个sender （共享底层通道），其次sender里实现了Arc 引用计数
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hi-2"),
            String::from("from-2"),
            String::from("the-2"),
            String::from("thread-2"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 将rx视为迭代器，不再显式调用recv. 并在通道关闭时退出循环
    for received in rx {
        println!("got:{}", received);
    }
}

/*
118 |     let counter = Mutex::new(0);
    |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
119 |     let mut handles = vec![];
120 |     for _ in 0..10 {
    |     -------------- inside of this loop
121 |         let hdl = thread::spawn(move || {
    |                                 ------- value moved into closure here, in previous iteration of loop
...
130 |     println!("result:{}", *counter.lock().unwrap());
    |                            ^^^^^^^ value borrowed here after move
    |
help: consider moving the expression out of the loop so it is only moved once
    |

上面的报错提示不明显，主要是因为使用了循环创建线程。根本原因是不应该将counter所有权移动到多个线程中
*/
fn t_wrong_shared_mutex() { // P518
    let counter = Mutex::new(0);
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _ in 0..10 {
        // let hdl = thread::spawn(move || { // 如果不move, 也会报：借来的 counter 引用可能在closure外失效
        //     let mut num = counter.lock().unwrap();
        //     *num += 1;
        // });
        // handles.push(hdl);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("t_wrong_shared_mutex result:{}", *counter.lock().unwrap()); // 一旦move进thread，则这里不能再borrow了： value borrowed here after move
}

// `Rc<Mutex<i32>>` cannot be sent between threads safely
// help: within `{closure@src\ch16_concurrency.rs:157:33: 157:40}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`

// 前面章节提到过Rc<>只用于单线程
fn t_wrong_shared_mutex_using_Rc() {
    let counter: Rc<Mutex<i32>> = Rc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _ in 0..10 {
        // let cnt = Rc::clone(&counter); // clone时会增加计数
        // let hdl = thread::spawn(move || {
        //     let mut num = cnt.lock().unwrap();
        //     *num += 1;
        // });
        // handles.push(hdl);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("t_wrong_shared_mutex_using_Rc result:{}", *counter.lock().unwrap());
}

fn t_shared_mutex_using_Arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let cnt = Arc::clone(&counter);
        let hdl = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(hdl);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("t_shared_mutex_using_Arc result:{}", *counter.lock().unwrap());
}

fn t_mutex() {
    let m = Mutex::new(5);
    {
        // 申请锁失败时，在这里简单panic
        // 将num视作一个指向内部数据的可变引用，想修改必须先lock，没有其他方式可访问到内部的i32
        // lock() 实际返回的是MutexGuard智能指针
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    t_wrong_shared_mutex();
    t_wrong_shared_mutex_using_Rc();
    t_shared_mutex_using_Arc();
}

struct MpmcSender<T>(mpsc::Sender<T>);

impl<T> Clone for MpmcSender<T> {
    fn clone(&self) -> Self {
        MpmcSender(self.0.clone())
    }
}

impl<T> MpmcSender<T> {
    fn send(&self, msg: T) -> Result<(), mpsc::SendError<T>> {
        self.0.send(msg)
    }
}

struct MpmcReceiver<T>(Arc<Mutex<mpsc::Receiver<T>>>);

impl<T> Clone for MpmcReceiver<T> {
    fn clone(&self) -> Self {
        MpmcReceiver(Arc::clone(&self.0))
    }
}

impl<T> MpmcReceiver<T> {
    fn recv(&self) -> Result<T, mpsc::RecvError> {
        let guard = self.0.lock().unwrap();
        guard.recv()
    }
}

fn create_mpmc_channel<T>() -> (MpmcSender<T>, MpmcReceiver<T>)
    where T: Send + 'static {
    let (tx, rx) = mpsc::channel();
    let arcRx = Arc::new(Mutex::new(rx));
    (MpmcSender(tx), MpmcReceiver(arcRx))
}

// from deepseek answer ; crossbeam-channel 库提供高阶版本的channel
/*
何时选择什么？

    标准库 mpsc: 大多数用例，特别是单消费者场景
    crossbeam-channel: 需要 MPMC 或更高级特性
    flume: 需要更友好的 API 和良好性能
这种设计让 Rust 标准库保持精简，同时通过生态系统提供丰富的选择。
 */
fn t_mpmc() {
    let (tx, rx) = create_mpmc_channel();
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for i in 0..2 {
        let rx = rx.clone();
        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                println!("consumer {} got : {}", i, msg);
            }
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
}

pub fn t16_concurrency() {
    println!("------------------ [t16_concurrency]");
    t1();
    // t_join();
    // t_borrow_by_move();
    //t_use_channel();
    // t_send_multiple_to_receiver();
    // t_use_multiple_sender();
    // t_mutex();
    //t_mpmc();
}