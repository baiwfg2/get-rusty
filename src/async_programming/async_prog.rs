use super::ch3_syscall;
use super::ch4_event_queue;
use super::ch5_fiber;
use super::ch7_coroutine;

pub fn t_async_main() {
    //ch3_syscall::t3_main();
    //ch4_event_queue::t4_main();
    //ch5_fiber::ch5();
    ch7_coroutine::t_coroutine_main();
}