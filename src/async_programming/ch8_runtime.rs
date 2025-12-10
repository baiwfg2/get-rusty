
use super::ch7_future::{Future, PollState};
use super::ch7_http::Http;

/////////// 我这个组织和书中不一样，用corofy时报错。直接使用书中main.rs后的代码

fn main() {
    let future = async_main();
    let mut runtime = Runtime::new();
    runtime.block_on(future);
}

coroutine fn async_main() {
    println!("program with `coroutine and wait` starting...");
    let txt = Http::get("/600/helloworld1").wait;
    println!("{txt}");
    let txt2 = Http::get("/400/helloworld2").wait;
    println!("{txt2}");
}