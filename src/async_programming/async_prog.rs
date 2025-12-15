use super::ch3_syscall;
use super::ch4_event_queue;
use super::ch5_fiber;
use super::ch7_intro_coroutine::ch7_entrypoint;
use super::ch8_reactor_executor::ch8_entrypoint_native_runtime;

use super::ch8_reactor_executor::entrypoint;

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
    ch7_entrypoint::t_coroutine_main();
    //ch8_entrypoint_native_runtime::t_run_coro_with_mioPoll();
    //entrypoint::t_run_reactor_executor();
}