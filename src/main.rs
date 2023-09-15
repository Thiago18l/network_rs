mod config;
mod server;

fn main() {
    let result: bool = config::ip::is_valid_ip("10.0.0.1");
    println!("the result of is_valid_ip: {}", result);
    config::config::accept_connections(120);
    let response = server::stream::stream_fn();
    println!("{:#?}", response);
    server::server::server();
}
