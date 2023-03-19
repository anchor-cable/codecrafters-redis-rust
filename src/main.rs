use std::{net::TcpListener, io::{BufReader, BufWriter, BufRead, Write}, thread};
use std::io::Result;

fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection Start!");
                thread::spawn(move || {
                    handle_stream(stream);
                });
                println!("Connection Closed!");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_stream(stream: std::net::TcpStream) {
    loop {
        let mut msg = String::new();
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        if let Ok(read_bytes) = reader.read_line(&mut msg) {
            if read_bytes == 0 {break;}
        }
        println!("receive:{}", msg);
        msg.clear();
        writer.write_all(b"+PONG\r\n").expect("unable to write");
        writer.flush().expect("unable tp flush");
    }
}