use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

pub fn run() {
    // 绑定 127.0.0.1:7878 进行监听
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 遍历监听的 stream
    for stream in listener.incoming() {
        // 由 handle_connection 处理 stream
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 新建 buffer 用于存储 stream 的内容
    let mut buffer = [0; 32];
    loop {
        // 读取 stream 的内容到 buffer
        let len = stream.read(&mut buffer).unwrap();
        // 将 buffer 中的有效内容转为 str
        let content = match str::from_utf8(&buffer[..len]) {
            // 转换成功直接返回
            Ok(s) => s,
            // 转换失败
            Err(e) => {
                // 打印错误信息
                println!("failed: {}", e);
                // 重新读取 stream
                continue;
            }
        };
        // server 打印内容
        println!("client send: {}", content);
        // 返回给 client 的消息
        stream
            .write(format!("server got: {}\n", content).as_bytes())
            .unwrap();
        // 压入 stream
        stream.flush().unwrap();
    }
}
