use std::io::{Result, Read};

pub fn handle_connection(socket: &mut std::net::TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];

    let bytes_read = socket.read(&mut buffer)?;

    let received_data = String::from_utf8_lossy(&buffer[0..bytes_read]);

    println!("Received: {}", received_data);

    Ok(())
}