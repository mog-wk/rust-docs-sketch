use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    println!("Hello, world!");
    shared_state_concurrency();
}

fn thread_example() {
    let v: Vec<i32> = vec![1, 2, 3, 4];

    // thread example
    let spawn_thread: thread::JoinHandle<_> = thread::spawn(|| {
        for i in 0..10 {
            println!("{} iteration of spawned thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let vec_thread: thread::JoinHandle<_> = thread::spawn(move || {
        for i in v {
            println!("{}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 0..5 {
        println!("{} iteration of main thread", i);
        thread::sleep(Duration::from_millis(100));
    }

    spawn_thread.join().unwrap();
    vec_thread.join().unwrap();
    println!("end func");
}

fn transfer_data_between_threads() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let s_v = vec![
            String::from("1-this"),
            String::from("1-is"),
            String::from("1-an"),
            String::from("1-iter"),
        ];
        for s in s_v {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    thread::spawn(move || {
        let s_v = vec![
            String::from("2-there"),
            String::from("2-are"),
            String::from("2-more"),
            String::from("2-transmitters"),
        ];
        for s in s_v {
            tx1.send(s).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for r in rx {
        println!("{} ", r);
    }
}
fn shared_state_concurrency() {

}
