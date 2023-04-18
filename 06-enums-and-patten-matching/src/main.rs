
#[derive(Debug)]
enum IpType {
    V4,
    V6,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
struct IpAddr {
    ty: IpType,
    address: String,
}

impl IpAddr {
    fn new(ty: IpType, address: String) -> Self {
        Self { ty, address }
    }
}

fn main() {
    println!("Hello, world!");

    let home = IpAddr {
        ty: IpType::V4,
        address: String::from("127.0.0.1"),
    };

    let work = IpAddr::new(IpType::V6, String::from("532.0.0.1"));

    println!("{:?} {:?}", home, work);
    println!("{:?} {:?}", coin_in_cents(Coin::Dime), coin_in_cents(Coin::Quarter));

}

fn coin_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
