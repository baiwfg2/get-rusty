use super::future::{Future, PollState};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, Thread},
};

#[derive(Clone)]
pub struct Waker {
    thread: Thread,
    taskId: usize,
    ready_queue: Arc<Mutex<Vec<usize>>>,
}

impl Waker {
    pub fn wake(&self) {
        self.ready_queue.lock()
            .map(|mut q| q.push(self.taskId)).unwrap();
        /*
        Every thread is equipped with some basic low-level blocking support, via the
         thread::park function and thread::Thread::unpark method. park blocks the
         current thread, which can then be resumed from another thread by calling
         the unpark method on the blocked thread’s handle.

         The unpark method on a Thread atomically makes the token available if it wasn’t already
         */
        self.thread.unpark();
    }
}

type Task = Box<dyn Future<Output = String>>;

thread_local! {
    static CURRENT_EXEC: ExecutorCore = ExecutorCore::default();
}

#[derive(Default)]
struct ExecutorCore {
    // We can’t simply mutate a static variable, so we need internal mutability here
    // Since this will only be callable from one
    // thread, a RefCell will do so since there is no need for synchronization.
    tasks: RefCell<HashMap<usize, Task>>,
    /*
    An Arc<…> (shared reference)
    to this collection will be given to each Waker that this executor creates. Since the Waker can
    (and will) be sent to a different thread and signal that a specific task is ready by adding the
    task’s ID to ready_queue, we need to wrap it in an Arc<Mutex<…>>.
     */
    ready_queue: Arc<Mutex<Vec<usize>>>,
    // Since the executor instance will only be accessible on the same thread it
    // was created, a simple Cell will suffice in giving us the internal mutability we need
    next_id: Cell<usize>,
}

pub fn spawn<F>(future: F)
    where F: Future<Output = String> + 'static {
    CURRENT_EXEC.with(|e| {
        let id = e.next_id.get();
        e.tasks.borrow_mut().insert(id, Box::new(future));
        e.ready_queue.lock().map(|mut q| q.push(id)).unwrap();
        e.next_id.set(id + 1);
    });
}

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    fn pop_ready(&self) -> Option<usize> {
        // LIFO queue
        CURRENT_EXEC.with(|q| q.ready_queue.lock()
            .map(|mut q| q.pop())
            .unwrap())
    }

    fn get_future(&self, id: usize) -> Option<Task> {
        // if the task not-ready, we need to add it back
        CURRENT_EXEC.with(|q| q.tasks.borrow_mut().remove(&id))
    }

    fn get_waker(&self, id: usize) -> Waker {
        Waker {
            taskId: id,
            thread: thread::current(),
            ready_queue: CURRENT_EXEC.with(|q| q.ready_queue.clone()),
        }
    }

    fn insert_task(&self, id: usize, task: Task) {
        CURRENT_EXEC.with(|q| {
            q.tasks.borrow_mut().insert(id, task);
        });
    }

    fn task_count(&self) -> usize {
        CURRENT_EXEC.with(|q| q.tasks.borrow().len())
    }

    pub fn block_on<F>(&mut self, future: F)
        where F: Future<Output = String> + 'static {
        spawn(future);
        loop {
            while let Some(id) = self.pop_ready() {
                let mut fut = match self.get_future(id) {
                    Some(f) => f,
                    // guard against false wakeups
                    None => continue,
                };

                let waker = self.get_waker(id);
                match fut.poll(&waker) {
                    PollState::NotReady => { self.insert_task(id, fut); }
                    PollState::Ready(_) => continue, // the current fut will be dropped
                }
            }

            let task_count = self.task_count();
            let name = thread::current().name().unwrap_or_default().to_string();

            if task_count > 0 {
                println!("{name}: {task_count} tasks remaining, sleep until notified");
                // yield control to OS sheduler
                thread::park();
            } else {
                println!("{name}: all tasks completed");
                break;
            }
        }
    }
}