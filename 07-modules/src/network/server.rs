
pub fn connect() {
    println!("server connect");
}

pub struct Test {
    x: u8,
    y: u8,
}

pub trait TestGen {
    fn new() -> Test;
}

impl TestGen for Test {
    fn new() -> Test {
        Test {x: 6, y: 2}
    }
}
