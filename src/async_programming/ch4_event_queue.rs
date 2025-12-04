use std::{env, io::{self, Read, Result, Write}, net::TcpStream};

use crate::async_programming::ffi;

use super::ffi::Event;
use super::poll::Poll;

use super::delay_service;

/*
In this specific instance with nConnection, most web servers are designed to be lenient
 and would likely ignore the unknown header and process the request anyway, so
 the delay function would probably still have been called. However,
 it's definitely a bug that's now fixed.
*/
fn get_req(path: &str) -> String {
    // 末尾的 \ 很重要，否则格式错误报 BAD REQUEST
    format!("GET {path} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n\
        \r\n"
    )
}

pub fn t4_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        delay_service::t_delayService()?;
        return Ok(());
    }
    let mut poll = Poll::new()?;
    let n_events = 5;
    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let req = get_req(&url_path);

        let mut stream = TcpStream::connect(addr)?; // default enable Nagle
        stream.set_nonblocking(true)?;
        stream.write_all(req.as_bytes())?;

        poll.registry().register(
            &stream,
            i,
            ffi::EPOLLIN | ffi::EPOLLET
        )?;
        streams.push(stream);
    }

    let mut handled_events = 0;
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("timeout or spurious wakeup");
            continue;
        }
        handled_events += handle_events(&events, &mut streams)?;
    }
    println!("finnished");
    Ok(())
}

fn handle_events(events: &[Event], streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_eve = 0;
    for e in events {
        let idx = e.token();
        let mut data = vec![0u8; 4096];
        // 可能会读多次. Remember how important it is to fully drain the buffer when using epoll in edge-triggered mode.
        loop {
            match streams[idx].read(&mut data) {
                // 对端关闭了连接（EOF）,永远不会再有数据了
                Ok(n) if n == 0 => {
                    handled_eve += 1;
                    println!("connection closed by peer: {:?}", e);
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("received:{:?}", e);
                    println!("{txt}\n-------------\n");
                }
                // not ready to read in a non-blocking manner. could happen even
                // if the event was reported as ready
                // 暂时没数据（但连接还活着）但响应可能还没传完, 不增加 handled_eve，等下次 epoll 再读
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
    }
    Ok(handled_eve)
}