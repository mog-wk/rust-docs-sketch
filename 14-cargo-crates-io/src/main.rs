// run cargo doc to generate html document
//! series of utilities that makes calculations more convenient
/// Documentation comment 
///
/// # Exemples
/// ...
/// let five = 5;
///
/// assert_eq(add_one(five), 6);
/// ...


use cargo_crates_io::colors;

pub fn add_one(x: i32) -> i32 { x + 1 }

fn main() {
    println!("Hello, world!");
    let red = colors::Primary::Red;
}


#[cfg(test)]
mod tests {
    use crate::add_one;
    #[test]
    fn test_add_one() {
        assert_eq!(add_one(5), 6);
        assert_eq!(add_one(2), 3);
        assert_ne!(add_one(2), 6);
    }
}
