const MAX_CONNECTIONS: u32 = 100;

#[allow(dead_code)]
fn config() {}

pub fn accept_connections(num_connections: u32) {
    if num_connections > MAX_CONNECTIONS {
        println!("Too many connections, maximum is allowed is {}", MAX_CONNECTIONS);
    } else {
        println!("Connection accepted!");
    }
}