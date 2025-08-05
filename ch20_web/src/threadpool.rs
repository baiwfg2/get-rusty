use std::{sync::mpsc, thread};
use std::sync::{Arc, Mutex};

// main中不感知Worker，所以保持私有
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker { id, thread }
    }
}

pub struct ThreadPool {
    // 为了更多灵活控制，引入中间层Worker，Worker里再包JoinHandle
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

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

    }
}