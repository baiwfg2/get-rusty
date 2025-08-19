use std::{sync::mpsc, thread};
use std::sync::{Arc, Mutex};

// main中不感知Worker，所以保持私有
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // 返回的LockGuard在let job 这个语句结束后就会自动 unlock
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
            }

            // BAD !!! unlock 逾期，job执行时，Lock仍在持有
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {} got a job; executing.", id);
            //     job();
            // }
        });
        Worker { id, thread }
    }
}

pub struct ThreadPool {
    // 为了更多灵活控制，引入中间层Worker，Worker里再包JoinHandle
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}