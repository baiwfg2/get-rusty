use std::time::Duration;
use std::{fs, thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

mod threadpool;
use threadpool::ThreadPool;

fn t1_simple() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}

fn handle_conn(mut stream: TcpStream) {
    // 512 bytes bufffer
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // lossy 表示遇到无效的utf8序列时，会用黑底 ? 来表示
    //println!("request:{}", String::from_utf8_lossy(&buffer[..]));
    //let response = "HTTP/1.1 200 OK\r\n\r\n";

    let contents = fs::read_to_string("ch20_web/hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_conn2(mut stream: TcpStream) {
    // 512 bytes bufffer
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {

        let contents = fs::read_to_string("ch20_web/hello.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("ch20_web/404.html").unwrap();
        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

//// 对handle_conn2的重构,并加入sleep逻辑(Page658)
fn handle_conn3(mut stream: TcpStream) {
    // 512 bytes bufffer
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "ch20_web/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "ch20_web/hello.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "ch20_web/404.html")
    };

    // println!("Current working directory: {:?}", std::env::current_dir().unwrap());
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn t2_read_request() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn3(stream);
    }
}

fn t3_threadpool() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_conn3(stream);
        });
    }
}

pub fn t20_webserver_main() {
    //t2_read_request();
    t3_threadpool();
}