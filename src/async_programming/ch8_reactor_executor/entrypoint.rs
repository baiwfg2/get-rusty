use super::http;
use super::runtime::{self, Waker};

use super::future::{Future, PollState};

use std::fmt::Write;

pub fn t_run_reactor_executor() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}


// =================================
// We rewrite this:
// =================================

// coroutine fn async_main() {
//     println!("Program starting");
//     let txt = http::Http::get("/600/HelloAsyncAwait").wait;
//     println!("{txt}");
//     let txt = http::Http::get("/400/HelloAsyncAwait").wait;
//     println!("{txt}");

// }

// =================================
// Into this:
// =================================

fn async_main() -> impl Future<Output=String> {
    Coroutine0::new()
}

enum State0 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

#[derive(Default)]
struct Stack0 {
    counter: Option<usize>,
    buffer: Option<String>,
    writer: Option<*mut String>,
    // writer: Option<&mut String>, //  expected named lifetime parameter
}

struct Coroutine0 {
    stack: Stack0,
    state: State0,
}

impl Coroutine0 {
    fn new() -> Self {
        Self { state: State0::Start, stack: Stack0::default() }
    }
}


impl Future for Coroutine0 {
    type Output = String;

    fn poll(&mut self, waker: &Waker) -> PollState<Self::Output> {
        loop {
        match self.state {
                State0::Start => {
                    self.stack.counter = Some(0);
                    self.stack.buffer = Some(String::from("\nBUFFER:\n----\n"));
                    // cast &mut was coerced to a *mut pointer
                    self.stack.writer = Some(self.stack.buffer.as_mut().unwrap());
                    // ---- Code you actually wrote ----
                    println!("Program starting");

                    // ---------------------------------
                    let fut1 = Box::new( http::Http::get("/600/hello1"));
                    self.state = State0::Wait1(fut1);
                }

                State0::Wait1(ref mut f1) => {
                    match f1.poll(waker) {
                        PollState::Ready(txt) => {
                            let mut counter = self.stack.counter.take().unwrap();
                            let writer = unsafe { &mut *self.stack.writer.take().unwrap() };
                            writeln!(writer, "{txt}");
                            counter += 1;

                            // ---------------------------------
                            let fut2 = Box::new( http::Http::get("/400/hello2"));
                            self.state = State0::Wait2(fut2);

                            // save stack
                            self.stack.counter = Some(counter);
                            self.stack.writer = Some(writer as *mut String);
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Wait2(ref mut f2) => {
                    match f2.poll(waker) {
                        PollState::Ready(txt) => {
                            let writer = unsafe { &mut *self.stack.writer.take().unwrap() };
                            let mut counter = self.stack.counter.take().unwrap();
                            counter += 1;
                            writeln!(writer, "{txt}").unwrap();

                            println!("Total requests: {}", counter);

                            let buffer = self.stack.buffer.as_ref().unwrap();
                            println!("{buffer}");


                            // ---------------------------------
                            self.state = State0::Resolved;
                            /*
                            gets dropped at the end of this scope too. If we didn’t do that, we would hold on to
                            the memory that’s been allocated to our String until the entire coroutine is dropped
                            (which could be much later).
                             */
                            let _ = self.stack.buffer.take().unwrap();
                            break PollState::Ready(String::new());
                        }
                        PollState::NotReady => break PollState::NotReady,
                    }
                }

                State0::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}

