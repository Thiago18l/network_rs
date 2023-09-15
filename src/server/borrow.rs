use log::{error, info};
use std::net::TcpListener;
use std::thread;
use crate::utils::utils::handle_connection;

pub fn borrow_style() {
    let listener = 
        TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                thread::spawn(move || {
                    let result = handle_connection(&mut stream.try_clone().unwrap());
                    info!("RECEIVED DATA: {:#?}", result);
                });
            }
            Err(error) => {
                error!("{:?}", error);
            }
        }
    }
}