
mod lib;
use lib::AvarageCollection;

fn main() {
    println!("Hello, world!");
    let mut avg_col = AvarageCollection::new(vec![1, 3, 5, 6, 7]);
    println!("{}", avg_col);
}
