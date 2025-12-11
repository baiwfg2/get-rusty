// P209 new runtime implementation

pub use super::executor::{Executor, spawn, Waker};
pub use super::reactor::reactor;

use super::reactor;

pub fn init() -> Executor {
    reactor::start();
    Executor::new()
}
