pub mod server;

pub fn connect() {
    println!("network connect");
}

#[derive(Debug)]
pub enum IpType {
    V4,
    V6,
}
