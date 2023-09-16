mod config;
mod server;
mod utils;
mod core;
mod enums;
mod interface;
use log::{info, error};

use crate::interface::tcp::NetworkProtocol;

fn main() {
    log4rs::init_file("../log4rs.yaml", Default::default()).unwrap();
    core::tcp::socket_tcp();
    enums::enums::process_message();

    let mut tcp_connection = interface::tcp::TcpProtocol::new();
    let address = "127.0.0.1:8081";

    match tcp_connection.connect(address) {
        Ok(()) => {
            info!("Dados enviados com sucesso");
            let mut buffer_r = [0; 1024];
            match tcp_connection.receive(&mut buffer_r) {
                Ok(bytes_received) => {
                    info!("dados recebidos, {}", bytes_received);
                }
                Err(err) => {
                    info!("Erro ao enviar dados: {}", err);
                }
            }
        }
        Err(err) => {
            error!("Erro na conexão, {}", err);
        }
    }
}

#[allow(dead_code)]
fn dead_code() {
    let result: bool = config::ip::is_valid_ip("10.0.0.1");
    info!("the result of is_valid_ip: {}", result);
    server::borrow::borrow_style();
    config::config::accept_connections(120);
    server::owner_sv::owner();
    let response = server::stream::stream_fn();
    info!("{:#?}", response);
    server::server::server();
}