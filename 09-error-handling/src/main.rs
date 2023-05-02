use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    // panic!

    //panic!("panic here");

    // Result<T, E>

    let f1 = match File::open("hello.txt") {
        Ok(f) => f,
        Err(error) => panic!("cound not open file: {:?}", error),
    };
    let f2 = match File::open("./hel.txt") {
        Ok(f) => f,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hel.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("could not create file: {:?}", e),
            }
        },
        
        Err(error) => panic!("could not open file: {:?}", error),
    };

    // shortcuts: unwrap and expect

    let f3 = File::open("hello.txt").unwrap(); // same as f1
    let f4 = File::open("hello.txt").expect("a panic message"); // same as f1 with custon panic message

    let username = match read_username("hello.txt") {
        Ok(s) => s,
        Err(e) => String::from("could not read file"),
    };
    println!("{:?}", &f1);
    println!("{:?}", &f2);
    println!("{:?}", &f3);
    println!("{:?}", &f4);
    println!("{:?}", username);

    // custom type validation
    // check if n is > 0 and < 100

    pub struct Guess {
        value: i32
    }

    impl Guess {
        pub fn new(value: i32) -> Self {
            if value < 0 || value > 100 {
                panic!("value out of bounds: {:?}", value);
            }
            Self { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let n1: Guess = Guess {value: 1233};
    //let n2: Guess = Guess::new(1223);
    let n3 = n1.value();

}


// propagation
fn read_username(st: &str) -> Result<String, io::Error> {
    let mut f = match File::open(st) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    /*
     * match f.read_to_strng(&mut s) {
     *  Ok(_) => Ok(s),
     *  Err(e) => Err(e),
     *  }
     */
    f.read_to_string(&mut s)?;
    Ok(s)
}
