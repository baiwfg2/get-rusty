use super::future::{Future, PollState};
use super::runtime::{self, reactor, Waker};
use mio::{Interest, Token};
use std::io::{Read, Write, ErrorKind};

fn get_req(path: &str) -> String {
    format!("GET {path} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n\
        \r\n"
    )
}

pub struct Http;

impl Http {
    pub fn get(path: &str) -> impl Future<Output = String> {
        HttpGetFuture::new(path)
    }
}

struct HttpGetFuture {
    stream: Option<mio::net::TcpStream>, // we con't connect to the stream at the time we create this
    buffer: Vec<u8>,
    path: String,
    id: usize,
}

impl HttpGetFuture {
    fn new(path: &str) -> Self {
        Self {
            stream: None,
            buffer: vec![],
            path: path.to_string(),
            id: reactor().next_id(),
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

    fn poll(&mut self, walker: &Waker) -> PollState<Self::Output> {
        if self.stream.is_none() {
            println!("first poll - start operation");
            // lazy scheme, send the request after poll for the first time
            self.write_request();

            // mio require &mut
            runtime::reactor().register(self.stream.as_mut().unwrap(),
            Interest::READABLE, self.id);
            ///////// different from ch7
            //runtime::registry().register(self.stream.as_mut().unwrap(),Token(0), Interest::READABLE).unwrap();
            //return PollState::NotReady;
        }

        let mut buff = vec![0u8; 4096];
        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                // 对端关闭了连接（EOF）, 不会再有数据了
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    println!("peer closed, reply len: {}", s.len());
                    runtime::reactor().deregister(self.stream.as_mut().unwrap(), self.id);
                    break PollState::Ready(s.to_string());
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]); // concatanate ?
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    println!("WouldBlock - not ready yet"); // 在刚发请求后，立即读，必然走这里
                    runtime::reactor().set_waker(walker, self.id);
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