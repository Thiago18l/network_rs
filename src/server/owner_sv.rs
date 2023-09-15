use std::io::prelude::*;
use std::net::TcpListener;
use log::{info, error};

pub fn owner() {
    let result = main();
    info!("{:#?}", result);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    info!("Listening on port 8080");
    for stream in listener.incoming() {
        let mut stream = stream?;
        info!("New client connected: {:?}", stream.peer_addr()?);
        let mut buffer = [0; 1024];
        loop {
            let bytes_r = stream.read(&mut buffer)?;
            if bytes_r == 0 {
                break;
            }
            
            let output = stream.write_all(&buffer[..bytes_r]);
            match output {
                Ok(_) => {
                    info!("{:#?}", output);
                }
                Err(error) => {
                    error!("{:#?}", error);
                }
            }
        }
    }
    Ok(())
}