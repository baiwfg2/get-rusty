use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

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
    });

    // 将rx视为迭代器，不再显式调用recv. 并在通道关闭时退出循环
    for received in rx {
        println!("got:{}", received);
    }
}

fn t_use_multiple_sender() {
    let (tx, rx) = mpsc::channel();
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

fn wrong_shared_mutex() {
    let counter = Mutex::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let hdl = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(hdl);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("result:{}", *counter.lock().unwrap());
}
*/

// `Rc<Mutex<i32>>` cannot be sent between threads safely
// help: within `{closure@src\ch16_concurrency.rs:157:33: 157:40}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
/*
前面章节提到过Rc<>只用于单线程
fn wrong_shared_mutex_using_Rc() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let cnt = Rc::clone(&counter);
        let hdl = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(hdl);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("result:{}", *counter.lock().unwrap());
}
*/

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
    println!("result:{}", *counter.lock().unwrap());
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

    // wrong_shared_mutex();
    t_shared_mutex_using_Arc();
}

pub fn t16_concurrency() {
    println!("------------------ [t16_concurrency]");
    //t1();
    // t_join();
    //t_borrow_by_move();
    //t_use_channel();
    //t_send_multiple_to_receiver();
    //t_use_multiple_sender();
    t_mutex();
}