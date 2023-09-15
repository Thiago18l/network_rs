use std::io::prelude::*;
use std::net::TcpListener;

pub fn stream_fn() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; 1024];

            loop {
                let bytes_r = stream.read(&mut buffer)?;
                if bytes_r == 0 {
                    break;
                };
                let message = String::
                    from_utf8_lossy(&buffer[0..bytes_r]);
                    println!("Received {}", message);
            }
        }
    Ok(())
}