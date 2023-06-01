use std::thread;
use std::time::Duration;
use std::iter::zip;

use iterclosures::Cacher;

#[allow(unused_variables, dead_code)]
fn main() {
    println!("Hello, world!");

    let mut f = Cacher::new(|n| n + 2);

    println!("{:?}\n", f.value(3));

    let a1:[u8; 4] = [5, 4, 3, 2];

    let v1: Vec<u32> = vec![12, 23, 34, 45, 56];
    println!("{:?}", v1);
    let v_iter = v1.iter();
    println!("{:?}", v_iter);
    let v_map = v_iter.map(|x| x*x);
    println!("{:?}", v_map);
    let v2: Vec<_> = v_map.collect();
    println!("{:?}", v2);

    for (i, j) in zip(v1, v2) {
        println!("{} {}", i, j);
    }
}

#[allow(unused_variables, dead_code)]
fn expensive_calc(intensity: u32) -> u32 {
    println!("calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity + 2
}

#[allow(unused_variables, dead_code)]
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
