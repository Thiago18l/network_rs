use std::net::{IpAddr, Ipv4Addr};
use log::info;

struct TcpSocketAddr {
    ip: IpAddr,
    port: u16
}

impl TcpSocketAddr {
    fn to_string(&self) -> String {
        return format!("{}:{}", self.ip, self.port);
    }
}

pub fn socket_tcp() {

    let addr = TcpSocketAddr {
        ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 8080
    };
    
    info!("{}", addr.to_string());

}
