use core::arch::asm;

const SSIZE: isize = 48;

/*
it tells us that
the data will be represented in memory in this exact way so we write to the right field. The Rust ABI
makes no guarantee that they are represented in the same order in memory; however, the C-ABI does.
必须与switch 保存状态的顺序一致
*/
#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new,
    );
}

fn t_stack_swap() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];

    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        println!("top: {}, bottom: {}, align_bottom:{}", stack.as_mut_ptr() as usize, stack_bottom as usize, sb_aligned as usize);
        let hello_ptr = hello as *const u8;
        println!("Bytes of the `hello` function at {:p}:", hello_ptr);
        // orginal offset is -16, -7 can be OK too
        std::ptr::write(sb_aligned.offset(-7) as *mut u64, hello as u64);
        // if we use -6 here, segfault will occur
        ctx.rsp = sb_aligned.offset(-7) as u64;

        for i in 0..SSIZE {
            println!(
                "mem: {}, val: {}",
                sb_aligned.offset(-i as isize) as usize,
                *sb_aligned.offset(-i as isize)
            )
        }

        gt_switch(&mut ctx);
    }
}

mod fiber {

// enable the feature
#![feature(naked_functions)]

use std::{arch::{asm, naked_asm}, option};

use tokio::runtime;

const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;
const MAX_THREADS: usize = 4;
static mut RUNTIME: usize = 0;

pub struct Runtime {
    threads: Vec<Thread>,
    current: usize, // indicate which thread is currently running
}

impl Runtime {
    pub fn new() -> Self {
        let base_thread = Thread {
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Running,
        };

        let mut threads = vec![base_thread];
        let mut available_threads = (1..MAX_THREADS).map(|_| Thread::new()).collect();
        // Moves all the elements of other into self, leaving other empty.
        threads.append(&mut available_threads);
        Runtime {
            threads,
            current: 0,
        }
    }

    pub fn init(&self) {
        unsafe {
            let r_ptr = self as *const Runtime;
            // 是为了方便从代码任何地方访问当前运行时的实例，not recommended for production code
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn run(&mut self ) {
        while self.t_yield() {}
        std::process::exit(0);
    }

    // called when the task is done
    fn t_return(&mut self) {
        if self.current != 0 {
            self.threads[self.current].state = State::Available; // can accept other task
            self.t_yield();
        }
    }

    // if enable inline, The issue manifests itself by the runtime exiting before
    // all the tasks are finished.
    #[inline(never)]
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;

        // round-robin scheduling for simplicity
        while self.threads[pos].state != State::Ready {
            // 可以校验其他线程不可能是 RUNNING 状态
            pos += 1;
            if pos == self.threads.len() {
                pos = 0;
            }
            if pos == self.current {
                return false; // no other tasks
            }
        }
        // after loop, pos will be the thread that's ready to resume

        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }
        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;

        unsafe {
            let old = &mut self.threads[old_pos].ctx as *mut ThreadContext;
            let new = &self.threads[pos].ctx as *const ThreadContext;
            asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
        }
        self.threads.len() > 0 // prevent optimize out
    }

    pub fn spawn(&mut self, f: fn()) {
        let available = self.threads.iter_mut()
            .find(|t| t.state == State::Available)
            .expect("no available thread"); // If we run out of threads, we panic in this scenario, but there
                                            // are several (better) ways to handle that. We’ll keep things simple for now
        let size = available.stack.len();
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !15) as *mut u8;
            // guard is called as soon as f returns
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(s_ptr.offset(-32) as *mut u64, f as u64);
            available.ctx.rsp = s_ptr.offset(-32) as u64;
            /*
            Why do we need the skip function?
                Remember how we explained how the stack works? We want the f function to be the first to
                run, so we set the base pointer to f and make sure it’s 16-byte aligned. We then push the address
                to the skip function and lastly the guard function. Since, skip is simply one instruction,
                ret, doing this makes sure that our call to guard is 16-byte aligned so that we adhere to the
                ABI requirements.
             */
        }
        available.state = State::Ready;
    }
}

fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    }
}

#[naked]
// https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust/issues/31
unsafe extern "C" fn skip() {
    // options(noreturn) is implicitly implied
    naked_asm!("ret");
}

/*
This function is very unsafe, and it’s one of the places where we
make big shortcuts to make our example slightly simpler to understand. If we call this and our Runtime
is not initialized yet or the runtime is dropped, it will result in undefined behavior. However, making
this safer is not a priority for us just to get our example up and running
*/
pub fn yield_thread() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    }
}

#[naked]
#[no_mangle]
unsafe extern "C" fn switch() {
    naked_asm!(
        "mov [rdi + 0x00], rsp", // save rsp to old context
        "mov [rdi + 0x08], r15",
        "mov [rdi + 0x10], r14",
        "mov [rdi + 0x18], r13",
        "mov [rdi + 0x20], r12",
        "mov [rdi + 0x28], rbx",
        "mov [rdi + 0x30], rbp",
        "mov rsp, [rsi + 0x00]", // load rsp from new context
        "mov r15, [rsi + 0x08]",
        "mov r14, [rsi + 0x10]",
        "mov r13, [rsi + 0x18]",
        "mov r12, [rsi + 0x20]",
        "mov rbx, [rsi + 0x28]",
        "mov rbp, [rsi + 0x30]",
        "ret",
        /*
        By using this option, we tell the compiler to treat the assembly
        block as if it never returns, and we make sure that we never fall through
         the assembly block by adding a ret instruction ourselves.
         */
        //options(noreturn)
    );
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Available, // be ready to be assigned a task
    Running,
    Ready, // ready to move forward and resume execution
}

struct Thread {
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State,
}

impl Thread {
    fn new() -> Self {
        Thread {
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Available,
        }
    }
}

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

pub fn t_fiber() {
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("thread 1 starting");
        let id = 1;
        for i in 0..10 {
            println!("thread:{} counter: {}", id, i);
            yield_thread();
        }
        println!("thread 1 done");
    });
    runtime.spawn(|| {
        println!("thread 2 starting");
        let id = 2;
        for i in 0..15 {
            println!("thread:{} counter: {}", id, i);
            yield_thread();
        }
        println!("thread 2 done");
    });
    runtime.run();
}

}

pub fn ch5() {
    fiber::t_fiber();
    //t_stack_swap();
}

