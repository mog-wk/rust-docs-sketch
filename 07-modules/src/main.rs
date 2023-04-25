
mod lib;
mod client;
mod network;

#[derive(Debug)]
pub enum Colors {
    Red,
    Green,
    Blue,
}

use Colors::*;

fn main() {
    println!("hello world"); 
    println!("{}", lib::add(5, 4));

    client::connect();
    network::connect();
    network::server::connect();

    println!("{:?} {:?}", network::IpType::V4 , network::IpType::V6);
    println!("{:?} {:?} {:?}", Red, Green, Blue);
}
