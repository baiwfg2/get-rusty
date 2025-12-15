use super::ch7_future::{Future, PollState};

use std::{io::{ErrorKind, Read, Write}};

fn get_req(path: &str) -> String {
    format!("GET {path} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n\
        \r\n"
    )
}

pub struct Http;

impl Http {
    /*
error[E0716]: temporary value dropped while borrowed
   --> src/main.rs:104:53
    |
104 |                     let fut1 = Box::new( Http::get(&get_path(0)));
    |                                                     ^^^^^^^^^^^  - temporary value is freed at the end of this statement
    |                                                     |
    |                                                     creates a temporary value which is freed while still in use
105 |                     self.state = State0::Wait1(fut1);
    |                                                ---- coercion requires that borrow lasts for `'static`
    |
    = note: due to object lifetime defaults, `Box<dyn future::Future<Output = String>>` actually means `Box<(dyn future::Future<Output = String> + 'static)>`
note: this call may capture more lifetimes than intended, because Rust 2024 has adjusted the `impl Trait` lifetime capture rules

    explicitly state that the returned Future is 'static (meaning it doesn't borrow from the input).
    原书的edition是2021，所以不会报错。但在2024版中会报上面的错
*/
    pub fn get(path: &str) -> impl Future<Output = String> + 'static {
        HttpGetFuture::new(path)
    }
}

struct HttpGetFuture {
    stream: Option<mio::net::TcpStream>, // we con't connect to the stream at the time we create this
    buffer: Vec<u8>,
    path: String,
}

impl HttpGetFuture {
    fn new(path: &str) -> Self {
        Self {
            stream: None,
            buffer: vec![],
            path: path.to_string(),
        }
    }

    fn write_request(&mut self) {
        let stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
        stream.set_nonblocking(true).unwrap();
        let mut stream = mio::net::TcpStream::from_std(stream); // why to transform this ?
        stream.write_all(get_req(&self.path).as_bytes()).unwrap();
        self.stream = Some(stream);
    }
}

impl Future for HttpGetFuture {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        if self.stream.is_none() {
            println!("first poll - start operation");
            // lazy scheme, send the request after poll for the first time
            self.write_request();
            return PollState::NotReady;
        }

        let mut buff = vec![0u8; 4096];
        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                // 对端关闭了连接（EOF）, 不会再有数据了
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    break PollState::Ready(s.to_string());
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]); // concatanate ?
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    return PollState::NotReady;
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                }
                Err(e) => {
                    panic!("unexpected error: {}", e);
                }
            }
        }
    }
}