use std::time::Duration;
use std::thread;

use super::ch7_future::{Future, PollState};
use super::ch7_http::Http;

// stoppable/resumable task
struct Coroutine {
    state: State,
}

enum State {
    Start, // created but hasn't been polled yet
    /*
    When we call Http::get, we get a HttpGetFuture returned that we store in
    the State enum. At this point, we return control back to the calling function so it can do
    other things if needed.
    属于定制化的状态
     */
    Wait1(Box<dyn Future<Output = String>>),
    // The second call to Http::get is the second place where we’ll pass control back to
    // the calling function.
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

impl Coroutine {
    fn new() -> Self {
        Self {
            state: State::Start,
        }
    }
}

impl Future for Coroutine {
    type Output = ();
    // This is a special coroutine which contains 2 leaf futures
    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("coroutine starting");
                    let fut = Box::new(Http::get("/600/helloworld1"));
                    self.state = State::Wait1(fut);
                }
                // 如果没有ref,  cannot move out of `self.state` as enum variant `Wait1` which is behind a mutable reference
                // move occurs because `fut` has type
                //   `Box<dyn ch7_future::Future<Output = std::string::String>>`, which does not implement the `Copy` trait
                State::Wait1(ref mut fut) => match fut.poll() {
                    PollState::Ready(txt) => {
                        // If the future returns PollState::Ready together with our data, we know that we can
                        // execute the instructions that rely on the data from the first future and advance to the next state
                        println!("{txt}");
                        let fut2 = Box::new(Http::get("/400/helloworld2"));
                        self.state = State::Wait2(fut2);
                    }
                    PollState::NotReady => break PollState::NotReady, // one yield point
                }
                State::Wait2(ref mut fut2) => match fut2.poll() {
                     PollState::Ready(txt2) => {
                        println!("{txt2}");
                        self.state = State::Resolved;
                        break PollState::Ready(()) // tell the caller the entire coroutine is done
                    }
                    PollState::NotReady => break PollState::NotReady, // another yield point
                }
                State::Resolved => panic!("polled a resolved coroutine"),
            }
        }
    }
}

fn async_main() -> impl Future<Output = ()> {
    Coroutine::new()
}

pub fn t_coroutine_main() {
    let mut coro = async_main();
    loop {
        match coro.poll() {
            PollState::NotReady => {
                // the control is yielded back to us. we could do other work here, such as scheduling another task,
                // if we want to, but in our case, just print schedule other tasks.
                println!("schedule other tasks");
            }
            PollState::Ready(()) => break,
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
/*
coroutine starting
first poll - start operation
schedule other tasks
schedule other tasks
schedule other tasks
schedule other tasks
schedule other tasks
schedule other tasks
schedule other tasks
HTTP/1.1 200 OK
content-length: 11
connection: close
content-type: text/plain; charset=utf-8
date: Wed, 10 Dec 2025 07:43:02 GMT

helloworld1
first poll - start operation
schedule other tasks
schedule other tasks
schedule other tasks
schedule other tasks
schedule other tasks
HTTP/1.1 200 OK
content-length: 11
connection: close
content-type: text/plain; charset=utf-8
date: Wed, 10 Dec 2025 07:43:02 GMT

helloworld2

*/