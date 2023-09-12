use std::net::TcpListener;
use std::io::{Result, Read};

pub fn server() {
    let listener =
        TcpListener::bind("127.0.0.1:8080").unwrap();
        println!("Server at 8080");
    
    loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("New connection {}", addr);
                std::thread::spawn(move || {
                    handle_connection(socket)
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection {}", e);
            }
        }
        
    }
}

fn handle_connection(mut socket: std::net::TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];

    let bytes_read = socket.read(&mut buffer)?;

    let received_data = String::from_utf8_lossy(&buffer[0..bytes_read]);

    println!("Received: {}", received_data);

    Ok(())
}