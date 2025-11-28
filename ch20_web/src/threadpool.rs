use std::sync::mpsc::Receiver;
use std::{sync::mpsc, thread};
use std::sync::{Arc, Mutex};

// main中不感知Worker，所以保持私有
struct Worker {
    id: usize,
    //thread: thread::JoinHandle<()>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Mutex 结构体没有unlock方法 ，因为锁的所有权依赖于MutxexGuard<T>的生命周期
                // 返回的LockGuard在let job 这个语句结束后就会自动 unlock ,以确保 job() 执行时锁是被持有的
                /* 不能写成这样：
                while let Ok(job) = receiver.lock().unwrap().recv() {
                    job();
                }
                */
                let msg = receiver.lock().unwrap().recv().unwrap();
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }

            // BAD !!! unlock 逾期，job执行时，Lock仍在持有
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {} got a job; executing.", id);
            //     job();
            // }
        });
        Worker { id, thread: Some(thread) }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    // 为了更多灵活控制，引入中间层Worker，Worker里再包JoinHandle
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/* FnOnce: 只能调用一次的闭包 trait，无参数无返回值
 'static: 生命周期，表示闭包捕获的数据必须在整个程序运行期间有效
(P664 not knowing how long thread will run, so it must be static)
没有 dyn 时，编译器不知道这是 trait object 还是泛型
dyn 明确表示这是一个 trait object（运行时多态

Send - 标记 trait（不需要 dyn）：
    这是标记 trait（marker trait），只是给类型添加约束
    不定义具体行为，只是告诉编译器"这个类型可以跨线程传递"
    是对主 trait 的额外约束
*/
type Job = Box<dyn FnOnce() + Send + 'static>;

/*  这样写会报多种描述的错误：
impl Trait` in type aliases is unstable (因为 impl Trait 代表一个具体的但未命名的类型 而类型别名需要明确的类型)
unconstrained opaque type
`Job` must be used in combination with a concrete type within the same crate

type Job = Box<impl FnOnce() + Send + 'static>;
*/

impl ThreadPool {
    /// Creates a new ThreadPool with the specified number of threads.
    /// # Panics
    /// Panics if `size` is zero.
    pub fn new(sz: usize) -> ThreadPool {
        assert!(sz > 0);
        let (sender, receiver) = mpsc::channel();
        let sender2 = sender.clone();

        // Arc允许多线程共用，Mutex保证只有一个线程从接收端接收任务
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(sz);
        for id in 0..sz {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender,
        }
    }
    pub fn new_wrong(sz: usize) {
        assert!(sz > 0);
        //let (sender, receiver) = mpsc::channel();

        // 如果下方不直接用receiver，上面的语句会报：cannot infer type of the type parameter `T` declared on the function `channel`
        // 为了让此语句不注释掉，需要加::<T>
        let (sender, receiver) = mpsc::channel::<Message>();

        let mut workers: Vec<Receiver<Message>> = Vec::with_capacity(sz);
        for id in 0..sz {
            // Report: move occurs because `receiver` has type `std::sync::mpsc::Receiver<Message>`, which does not implement the `Copy` trait
            //workers.push(receiver);

            // Report: expected `Receiver<Message>`, found `&Receiver<_>`
            //workers.push(&receiver);

            // method not found in `std::sync::mpsc::Receiver<_>`
            // workers.push(receiver.clone());
        }
    }

    // FnOnce() 的() 表示没有参数，也没有返回值，不可省()
    // If omit 'static, report: the parameter type `F` must be valid for the static lifetime,so that the type `F` will meet its required lifetime bounds
    //   use 'static because we don't know how long the thread will take to execute (Eng ver P654)
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        let job2 = Box::new(12);
        //Message::NewJob(job2); //  expected an `FnOnce()` closure, found `{integer}
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    // support graceful shutdown
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 如果worker 不是 Option<> 类型，这样写会报错,join方法要求取得调用者的所有权
            // Error: cannot move out of `worker.thread` which is behind a mutable reference

            // Eng ver P670 says: move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
            // worker.thread.join().unwrap();

            // immitate what's been used in request_review
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }

            /*
            Eng ver介绍了新的方法, 更推荐
            fn drop(&mut self) {
                for worker in self.workers.drain(..) {
                    worker.thread.join().unwrap();
                }
             */
        }
    }
}