use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // 循环读取，兼容数据大小超过1028Byte的情况
    loop {
        // 开辟1028Byte Buffer
        let mut read = [0; 1028];
        // 模式匹配
        match stream.read(&mut read) {
            Ok(n) => {
                // 如果已经读到了数据的末尾，退出循环
                if n == 0 {
                    break;
                }
                // 打印请求字符
                println!("Request: {}", String::from_utf8_lossy(&read[..]));
                // 回写raw数据
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn main() {
    // 监听8080端口，用wrap处理端口占用等异常情况
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 端口接收到数据时的回调
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 多线程处理
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}
