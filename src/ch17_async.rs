use bigdecimal::num_traits::int;
use trpl::{Html, Either, ReceiverStream, Stream, StreamExt};
use log::info;
use std::sync::mpsc::Receiver;
use std::{time::Duration, vec};
use std::thread;
use std::{pin::pin};


async fn page_title(url: &str) -> Option<String> {
    let resp = trpl::get(url).await;
    let resp_text = resp.text().await; // 除了等待header，还要等待body全部接收到
    // let resp_text = trpl::get(url).await.text().await; // await写在后面方便chaining
    Html::parse(&resp_text)
        .select_first("title") // 返回Option，如果有map处理，如果没有map则do nothing
        .map(|title| title.inner_html())
}

fn t1() {
    let args: Vec<String> = std::env::args().collect();
    // 以下两种写法都不行
    // match page_title(url) { // this expression has type `impl Future<Output = Option<String>>
    // match page_title(url).await { // only allowed inside `async` functions and blocks

    // run sets up a runtime that’s used to run the future passed in.
    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("Title: {}", title),
            None => println!("No title found"),
        }
    });
}

async fn page_title2(url: &str) -> (&str, Option<String>) {
    let resp = trpl::get(url).await;
    let resp_text = resp.text().await; // 除了等待header，还要等待body全部接收到
    // let resp_text = trpl::get(url).await.text().await; // await写在后面方便chaining

    std::fs::write("a.txt", &resp_text).expect("Unable to write file");

    let title = Html::parse(&resp_text)
        .select_first("title") // 返回Option，如果有map处理，如果没有map则do nothing
        .map(|title| title.inner_html());
    (url, title)
}

fn t2() {
    let args: Vec<String> = std::env::args().collect(); // type must be known at this point
    trpl::run(async {
        let title1 = page_title2(&args[1]);
        let title2 = page_title2(&args[2]);
        let (url, maybe_title) =
            match trpl::race(title1, title2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("first page is: '{title}'"),
            None => println!("no title found"),
        }
    })
    // 访问 163.com 返回 <title>403 Forbidden</title> ,大概是因为要自定义header
}

fn t3_use_async_spawn_task() {
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("from 1st task {i}");
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("from 2nd task {i}");
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }
    })
}

fn t4_use_async_spawn_task() {
    trpl::run(async {
        /*
        The provided future will start running in the background immediately when spawn is called,
        even if you don't await the returned JoinHandle.
        Spawning a task enables the task to execute concurrently to other tasks. The spawned task may
         execute on the current thread, or it may be sent to a different thread to be executed.
         The specifics depend on the current Runtime configuration.
         */
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("from 1st task {i}");
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("from 2nd task {i}");
            trpl::sleep(std::time::Duration::from_millis(200)).await;
        }
        // we can use await to do the same thing, because the task handle itself is a future.
        // Its Output type is a Result , so we also unwrap it after awaiting it.
        handle.await.unwrap();
    })
}

fn t5_join_two_futures() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("from 1st task {i}");
                trpl::sleep(std::time::Duration::from_millis(200)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("from 2nd task {i}");
                trpl::sleep(std::time::Duration::from_millis(200)).await;
            }
        };
        // This function will return a new future which awaits both futures to complete.
        // The returned future will finish with a tuple of both results.
        // Note that this function consumes the passed futures and returns a wrapped version of it.
        trpl::join(fut1, fut2).await;
    })
}

fn t6_use_async_channel() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let vals = vec!["hello", "from", "the", "other", "side"];
        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // 上下两块并不是并发执行的，所有必须会等发送完毕才会执行接收
        while let Some(value) = rx.recv().await {
            info!("got: {}", value); // 过了2s 全部打印，而不是每隔500ms打印一个
            //println!("got: {}", value);
        }
    });
}

fn t7_use_async_channel_with_join() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx_fut = async {
            let vals = vec!["hello", "from", "the", "other", "side"];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            // 上下两块现在是并发执行的
            while let Some(value) = rx.recv().await {
                info!("got: {}", value); // 现在每隔500ms打印一个
            }
        };
        trpl::join(tx_fut, rx_fut).await;
    });
}

fn t8_use_async_channel_with_join_and_move() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        // move进后，sender 就可以自动在发送任务结束后被丢弃，于是rx 就能退出循环
        let tx1_fut = async move {
            let vals = vec!["hello", "from", "the", "other", "side"];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(200)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                info!("got: {}", value);
            }
        };

        let tx_fut = async move {
            let vals = vec!["more", "messages", "for", "you"];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(600)).await;
            }
        };

        // let futures_boxed = vec![Box::new(tx1_fut), Box::new(tx_fut), Box::new(rx_fut)]; // unmatched types

        // 显式指定类型后，可消除unmatched types错误（但为何能消除？）, 但仍有：the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
        //let futures_boxed: Vec<Box<dyn Future<Output = ()>>> = vec![Box::new(tx1_fut), Box::new(tx_fut), Box::new(rx_fut)];

         // 使用 Box::pin 将不同类型的 future 转换为相同的 trait object 类型 (P492 有解释)
         // 也可以考虑在future define时用 pin!()
        let futures_pinned: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(tx_fut), Box::pin(rx_fut)];

        /* error[E0308]: mismatched types
        --> src/main.rs:195:37
            |
        170 |         let tx1_fut = async move {
            |                       ---------- the expected `async` block
        ...
        184 |         let tx_fut = async move {
            |                      ---------- the found `async` block
        ...
        195 |         let futures = vec![tx1_fut, tx_fut, rx_fut];
            |                                     ^^^^^^ expected `async` block, found a different `async` block
            |
            = note: expected `async` block `{async block@src/main.rs:170:23: 170:33}`
                    found `async` block `{async block@src/main.rs:184:22: 184:32}`
            = note: no two async blocks, even if identical, have the same type
            = help: consider pinning your async block and casting it to a trait object */
        //let futures = vec![tx1_fut, tx_fut, rx_fut];


        //trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        //trpl::join!(tx1_fut, tx_fut, rx_fut); // 支持可变数目的futures
        trpl::join_all(futures_pinned).await; // 支持传入实现了Iterator的集合
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms} ms");
}

fn t9_race_future() {
    trpl::run(async {
        let a = async {
            println!("a started");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis((50))).await;
            println!("a completed");
        };

        let b = async {
            println!("b started");
            slow("b", 70);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis((50))).await;
            println!("b completed");
        };

        trpl::race(a, b).await;
    });
    /* no interleaving between a and b
    a started
    a ran for 30 ms
    a ran for 10 ms
    a ran for 20 ms
    b started
    b ran for 70 ms
    b ran for 10 ms
    b ran for 15 ms
    b ran for 350 ms
    a completed
     */
}

fn t10_use_yield_to_pause_long_running_task() {
    trpl::run(async {
        let a = async {
            println!("a started");
            slow("a", 30);
            trpl::yield_now().await; // 比trpl::sleep 好，后者有精度问题
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("a completed");
        };

        let b = async {
            println!("b started");
            slow("b", 70);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("b completed");
        };

        trpl::race(a, b).await;
    });
}

// build own async abstractions
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    // race 的调度不是fair的，按照传入的顺序执行，所以即使duration很短， future_to_try也有机会执行 (P501)
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

////////// stream future
fn t11_stream_future() {
    use trpl::StreamExt; // for .next()

    let values = [1, 2, 3, 4, 5];
    let iter = values.iter().map(|n| n * 2);
    // next()要求 borrow as mutable
    let mut stream = trpl::stream_from_iter(iter);

    trpl::run(async {
        while let Some(val) = stream.next().await {
            println!("Got value from stream : {}", val); // P505
        }

        let mut msg = get_messages();
        while let Some(m) = msg.next().await {
            println!("recv from async channel by streaming: {}", m);
        }
    });
}

fn t11_stream_future_with_timeout() {
    use trpl::StreamExt; // for .next()

    trpl::run(async {
        // If not use pin!, error: within `(PhantomData<&()>, PhantomPinned)`, the trait `Unpin` is not implemented for `PhantomPinned`
        // P507: we pin the message after applying timeout to them, because the timeout produces a stream that needs to be pinned to be polled
        let mut msgs = pin!(
            get_messages_timeout().timeout(Duration::from_millis(200)));
        while let Some(m) = msgs.next().await {
            // 超时并不影响msg最终会到达，因为channel是unbounded，如果在超时前未到达 ，在下一次poll时，可能已经到了
            match m {
                Ok(msg) => println!("recv from async channel by streaming with timeout: {}", msg),
                Err(reason) => {
                    println!("timeout reached: {reason}");
                }
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    let msgs = ["hello", "from", "the", "other", "side"];
    for m in msgs {
        tx.send(format!("Message -> '{m}'")).unwrap();
    }
    ReceiverStream::new(rx)
}

fn get_messages_timeout() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    // 已经设置了runtime，可以使用。如果没有setup runtime，这里会panic
    trpl::spawn_task(async move {
        let msgs = ["hello", "from", "the", "other", "side"];
        for (idx, m) in msgs.into_iter().enumerate() {
            let time_to_sleep = if idx % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            tx.send(format!("Message -> '{m}'")).unwrap();
        }
    });
    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let mut cnt = 0;
        loop {
            trpl::sleep(Duration::from_millis(10)).await;
            cnt += 1;
            tx.send(cnt).unwrap();
        }
    });
    ReceiverStream::new(rx)
}
/////////// merge stream
fn t12_merge_stream() {
    use trpl::StreamExt; // for .next()

    trpl::run(async {
        let msg = get_messages_timeout().timeout(Duration::from_millis(200));
        //let intervals = get_intervals();
        // error: expected `Result<String, Elapsed>`, found `u32`
        //let merged = msg.merge(intervals);

        let intervals = get_intervals().map(|cnt| format!("Interval: {cnt}"))
            // interval stream没有timeout，但为了和msg 类型匹配，设置一个很大的timeout即可
            .timeout(Duration::from_secs(10));

        let merged_tmp = msg.merge(intervals).take(20);
        // 如果不pin,则在next时报：within `(PhantomData<&()>, PhantomPinned)`, the trait `Unpin` is not implemented for `PhantomPinned
        let mut merged = pin!(merged_tmp);
        while let Some(m) = merged.next().await {
            match m {
                Ok(msg) => println!("merge-stream , msg: {}", msg),
                Err(reason) => {
                    println!("msg-stream timeout reached: {reason}");
                }
            }
        }
    });
}

fn t12_merge_stream_with_throttle() {
    use trpl::StreamExt; // for .next()

    trpl::run(async {
        let msg = get_messages_timeout().timeout(Duration::from_millis(200));

        let intervals = get_intervals()
            .map(|cnt| format!("Interval: {cnt}"))
            // 限制interval stream overwhelm the messages stream. 它产生一个new stream，会控制original stream在throttle rate下poll
            // 线程就不合适这样 compose了
            .throttle(Duration::from_millis(40))
            .timeout(Duration::from_secs(10));

        let merged_tmp = msg.merge(intervals).take(20);
        let mut merged = pin!(merged_tmp);
        while let Some(m) = merged.next().await {
            match m {
                Ok(msg) => println!("merge-stream , msg: {}", msg),
                Err(reason) => {
                    println!("msg-stream timeout reached: {reason}");
                }
            }
        }
    });
}

fn t13_combine_thread_with_async() {
    let (tx, mut rx) = trpl::channel();
    thread::spawn(move || {
        for i in 1..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    trpl::run(async {
        // 和 stream.next一样，要求对象是可变的
        while let Some(m) = rx.recv().await {
            println!("recv from thread via async channel: {}", m);
        }
    });
}

pub fn t17_async() {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    //t1();
    //t2();
    //t3_use_async_spawn_task();
    //t4_use_async_spawn_task();
    //t5_join_two_futures();
    //t6_use_async_channel();
    //t7_use_async_channel_with_join();
    //t8_use_async_channel_with_join_and_move();
    //t9_race_future();
    //t10_use_yield_to_pause_long_running_task();
    //t11_stream_future();
    //t11_stream_future_with_timeout();
    //t12_merge_stream();
    //t12_merge_stream_with_throttle();
    //t13_combine_thread_with_async();
}
