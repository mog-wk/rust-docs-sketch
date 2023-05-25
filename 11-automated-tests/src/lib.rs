// cargo test
// cargo test -- --nocapture
// cargo test -- --test-threads=1   // runs test in a simgle thread
// cargo test -- --ignored    // runs all #[ignore] tests
// cargo test scalar    // runs only "scalar" test
// cargo test mult    // runs all test with \mult\


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scalar() {
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 1, 4);
    }
    #[test]
    fn mult_2() {
        assert_eq!(2 * 5, 10);
        assert_eq!(2 * 6, 12);
        assert_eq!(2 * 10, 20);
        assert_eq!(2 * 50, 100);
    }
    #[test]
    fn mult_5() {
        assert_eq!(5 * 5, 25);
        assert_eq!(5 * 6, 30);
        assert_eq!(5 * 10, 50);
    }
    #[test]
    fn rect() {
        let r1 = Rectangle::new(12, 12);
        let r2 = Rectangle::new(6, 6);
        let r3 = Rectangle::new(4, 16);

        assert_eq!(r1.area(), 144);
        assert_eq!(r2.area(), 36);
        assert_eq!(r3.area(), 64);

        assert_eq!(r1.can_hold(&r2), true);
        assert_ne!(r1.can_hold(&r3), true);
        assert_ne!(r3.can_hold(&r1), true);
        assert_ne!(r1.can_hold(&r1), true);
    }
    #[test]
    #[allow(unreachable_code)]
    #[should_panic]
    fn panic_test() {
        panic!();
        assert_eq!(4u32 + 4u16 as u32, 8u32);
    }
    
    #[test]
    #[ignore]
    fn ignored_test() {
        print!("this will be ignored");
    }
}



struct Rectangle {
    height: u32,
    width: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn new(width: u32, height:u32) -> Rectangle {
        Rectangle{ width, height }
    }
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl std::ops::Add<Rectangle> for Rectangle {
    type Output = Rectangle;
    fn add(self, other: Rectangle) -> Self::Output {
        Rectangle {
            height: self.height + other.height,
            width: self.width + other.width,
        }
    }
}

