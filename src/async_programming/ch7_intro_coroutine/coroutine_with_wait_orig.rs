// P178 按照作者的方法，此例子支持 coroutine, wait 关键字，分别对应 async, await

use super::ch7_future::{Future, PollState};
use super::ch8_http::Http;

fn get_path(i: usize) -> String {
    format!("/{}/HelloWorld{i}", i * 1000)
}

coroutine fn async_task() {
    println!("program with `coroutine and wait` starting...");
    let txt = Http::get(&get_path(0)).wait;
    println!("{txt}");
    let txt2 = Http::get(&get_path(1)).wait;
    println!("{txt2}");
    let txt3 = Http::get(&get_path(2)).wait;
    println!("{txt3}");
    let txt4 = Http::get(&get_path(3)).wait;
    println!("{txt4}");
    let txt5 = Http::get(&get_path(4)).wait;
    println!("{txt5}");
}

fn t_customized_coroutine_wait() {
    let start = std::time::Instant::now();
    let mut future = async_task();
    loop {
        match future.poll() {
            PollState::Ready(()) => break,
            PollState::NotReady => {
                // println!("not ready yet, do other things...");
                // do other things ...
            }
        }
    }
    println!("all done in {} ms", start.elapsed().as_secs_f32());
}