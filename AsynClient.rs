extern crate futures;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    //sync::mpsc,
};

#[tokio::main]
async fn main() {
    let s = TcpStream::connect("127.0.0.1:8888").await.unwrap();
    println!("Connect to server succ ...");
    let (s_reader, s_writer) = s.into_split();

    let mut read_task = tokio::spawn(async move {
        read_from_server(s_reader).await;
    });

    let mut write_task = tokio::spawn(async move {
        write_to_server(s_writer).await;
    });

    if tokio::try_join!(&mut read_task, &mut write_task).is_err() {
        eprintln!("read_task/write_task terminated");
        read_task.abort();
        write_task.abort();
    };
}


async fn read_from_server(reader: OwnedReadHalf) {
    let mut buf_reader = tokio::io::BufReader::new(reader);
    let mut buf = String::new();
    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(_e) => {
                eprintln!("read from client error");
                break;
            }
            
            Ok(0) => {
                println!("client closed");
                break;
            }
            Ok(n) => {
                // read_line()读取时会包含换行符，因此去除行尾换行符
                // 将buf.drain(。。)会将buf清空，下一次read_line读取的内容将从头填充而不是追加
                buf.pop();
                let content = buf.drain(..).as_str().to_string();
                println!("read {} bytes from client. content: {}", n, content);
                if content.trim().to_lowercase() == "bye".to_string(){
                    println!("Finished");
                    break
                }
            }
        }
    }
}

/// 写给客户端
async fn write_to_server(mut writer: OwnedWriteHalf) {
    loop{
        let mut input = String::new();
        
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    
        
        if let Err(e) = writer.write_all(input.as_bytes()).await{
            eprintln!("write to client failed: {}", e);
            break;
        }
        else{
            print!("Write to client: {}", input);
            if input.trim().to_lowercase() == "bye".to_string(){
                break
            }
        }
    }
}

/*use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use std::net::SocketAddr;
fn main() {
    let remote: SocketAddr = "127.0.0.1:8888".parse().unwrap();
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect to server");
    stream.set_read_timeout(Some(Duration::from_secs(3))).expect("Could not set a read timeout");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        stream.write(input.as_bytes()).expect("Failed to write to server");
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer);
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}*/
