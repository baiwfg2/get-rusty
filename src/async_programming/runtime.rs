use super::ch7_future::{Future, PollState};

use mio::{Events, Poll, Registry};
use std::sync::OnceLock;

static REGISTRY: OnceLock<Registry> = OnceLock::new();
pub fn registry() -> &'static Registry {
    REGISTRY.get().expect("Called outside a runtime context")
}

pub struct Runtime {
    poll: Poll,
}

impl Runtime {
    pub fn new() -> Self {
        let poll = Poll::new().unwrap();
        let registry = poll.registry().try_clone().unwrap();
        REGISTRY.set(registry).unwrap();
        Self { poll }
    }

    pub fn block_on<F>(&mut self, fut: F)
        where F: Future<Output = String> {
        let mut future = fut; // why not use fut directly?
        loop {
            match future.poll() {
                PollState::NotReady => {
                    println!("schedule other tasks\n");
                    let mut events = Events::with_capacity(10);
                    self.poll.poll(&mut events, None).unwrap();
                    // 直到有事件，poll才会返回
                }
                PollState::Ready(_) => break,
            }
        }
    }
}