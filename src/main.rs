mod config;
mod server;
use log::info;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("AQUI");
    let result: bool = config::ip::is_valid_ip("10.0.0.1");
    info!("the result of is_valid_ip: {}", result);
    config::config::accept_connections(120);
    server::owner_sv::owner();
    let response = server::stream::stream_fn();
    info!("{:#?}", response);
    server::server::server();
}
