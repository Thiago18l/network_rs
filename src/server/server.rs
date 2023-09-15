use std::net::TcpListener;
use crate::utils::utils::handle_connection;

pub fn server() {
    let listener =
        TcpListener::bind("127.0.0.1:8080").unwrap();
        println!("Server at 8080");
    
    loop {
        match listener.accept() {
            Ok((mut socket, addr)) => {
                println!("New connection {}", addr);
                std::thread::spawn(move || {
                    handle_connection(&mut socket)
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection {}", e);
            }
        }
        
    }
}
