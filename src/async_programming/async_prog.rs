use super::ch3_syscall;
use super::ch4_event_queue;
use super::ch5_fiber;
use super::ch7_coroutine;
use super::ch8_runtime_corofied;

use super::delay_service;

pub fn t_async_main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        delay_service::t_delayService().unwrap();
        return;
    }
    //ch3_syscall::t3_main();
    //ch4_event_queue::t4_main();
    //ch5_fiber::ch5_main();
    //ch7_coroutine::t_coroutine_main();
    ch8_runtime_corofied::t_run_coro_with_mioPoll();
}