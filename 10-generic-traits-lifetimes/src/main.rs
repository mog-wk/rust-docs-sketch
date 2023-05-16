use std::ops::Add;

mod assets;
use crate::assets::summary::Summary;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: std::ops::Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Self::Output {
        Self {x: p.x + self.x, y: p.y + self.y, }
    }
}

enum Color<R, G, B> {
    Blue(R, G, B),
    Red(R, G, B),
    White(R, G, B),
}


fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 2, 3, 4, 5];

    println!("{:?} {}", v1, largest(&v1));

    let int_point = Point { x: 1, y: 5 };
    let flo_point = Point { x: 1.23, y: 2.33 };

    let flo_point_I = Point::new(12.12, 45.23);

    println!("{:?} {:?}", int_point, flo_point);
    println!("{:?}", flo_point + flo_point_I);

    let tweet = assets::summary::Tweet {
        username: "pepega".to_string(),
        content: String::from("this is my first tweet :)"),
        reply: false, retweet: false 
    };
    println!("{}", tweet.summary());
}

#[warn(dead_code)]
fn largest<T: std::cmp::PartialOrd>(ls: &[T]) -> &T {
    let mut larg = &ls[0];
    for elt in ls.iter() {
        if elt > larg {
            larg = &elt;
        }
    }
    larg
}


