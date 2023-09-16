use std::{net::{TcpStream, SocketAddr}, time::Duration};
use std::io::*;
use std::result::Result;
use log::error;

enum ConnectionState {
    Listen,
    Established,
    FinWait,
    CloseWait,
    Closed,
}

pub struct TcpProtocol {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgment_number: u32,
    flags: u16,
    window_size: u16,
    options: u32,
    data: Vec<u32>,
    state: ConnectionState,
    stream: Option<TcpStream>
}

pub trait NetworkProtocol {
    fn connect(&mut self, address: &str) ->
        Result<(), Error>;
    
    fn send(&mut self, data: &[u8]) ->
        Result<(), Error>;
    
    fn receive(&mut self, buffer: &mut [u8]) ->
        Result<usize, Error>;
}

impl TcpProtocol {
    pub fn new() -> TcpProtocol {
      TcpProtocol {
        source_port: 0,
        destination_port: 0,
        sequence_number: 0,
        acknowledgment_number: 0,
        flags: 0,
        window_size: 0,
        options: 0,
        data: [0].to_vec(),
        state: ConnectionState::Closed,
        stream: None
      }
    }
}

impl NetworkProtocol for TcpProtocol {

    fn connect(&mut self, address: &str) ->
            Result<(), Error> {
        match address.parse::<SocketAddr>() {
            Ok(remote_addr) => {
                match TcpStream::connect_timeout(&remote_addr, Duration::from_secs(10)) {
                    Ok(stream) => {
                        self.stream = Some(stream);
                        self.state = ConnectionState::Established;
                        Ok(())
                    }
                    Err(err) => {
                        error!("Failed connection {}", err);
                        Err(Error::new(ErrorKind::ConnectionRefused, "Failed Connection"))
                    }
                }
            }
            Err(err) => Ok({
                error!("Invalid address {}", err);
            })
        }
    }

    fn send(&mut self, data: &[u8]) ->
            Result<(), Error> {
        match &mut self.stream {
            Some(stream) => {
                match stream.write(data) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        error!("Send data failed {}", err);
                        Err(Error::new(ErrorKind::Interrupted, "Failed Connection"))
                    }
                }
            }
            None => Err(Error::new(ErrorKind::NotConnected, "No connection estabilished"))
        }
    }
    fn receive(&mut self, buffer: &mut [u8]) ->
            Result<usize, Error> {
        match &mut self.stream {
            Some(stream) => {
                match stream.read(buffer) {
                    Ok(bytes_read) => {
                        Ok(bytes_read)
                    },
                    Err(err) => {
                        error!("Failed to receive data {}", err);
                        Err(Error::new(ErrorKind::Other, "Failed to receive data"))
                    }
                }
            }
            None => Err(Error::new(ErrorKind::ConnectionRefused, "No connection estabilished"))
        }
    }
}