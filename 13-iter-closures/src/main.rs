use std::thread;
use std::time::Duration;

use iterclosures::Cacher;

fn main() {
    println!("Hello, world!");

    let mut f = Cacher::new(|n| n + 2);

    println!("{:?}", f.value(3));
    println!("{:?}", f.value(4));
    println!("{:?}", f.display());
}

fn expensive_calc(intensity: u32) -> u32 {
    println!("calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity + 2
}

fn generate_trainning(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        num + 2
        });
    if intensity < 25 {
        println!("do {} push ups", expensive_result.value(intensity));
    } else {
        println!("do {} sits ups", expensive_result.value(intensity));
    }
}
