use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use dotenv::dotenv;

pub const SOCKET_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

pub fn get_database_url() -> String {
	dotenv().ok();
	std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
