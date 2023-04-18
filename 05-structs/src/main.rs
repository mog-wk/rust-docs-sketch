
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    index: u32,
    active: bool,
}

fn main() {
    println!("structs");

    // tuples
    let tup: (i32, &str, f64) = (12, "lul", 3.14);
    println!("{:?}", tup);

    // structs
    // instance definition
    let user1 = User {
        email: String::from("pogger@gmail.com"),
        username: String::from("user1"),
        index: 41,
        active: true,
    };

    // mut instance 
    let mut user2 = User {
        email: String::from("mogger@gmail.com"),
        username: String::from("user2"),
        index: 72,
        active: false,
    };
    user2.index = 16;

    // wrapper function
    let user3 = build_user(String::from("user3"), String::from("kogger@hotmail.com"), 15, true);

    // struct update syntax
    let user4 = User {
        email: String::from("logger@mail.com"),
        username: String::from("user4"),
        ..user1
    };

    // tuple structs
    #[derive(Debug)]
    struct Color(u32, u32, u32);
    let white = Color(0, 0, 0);

    println!("{:?}\n{:?}\n{:?}\n{:?}", user1, user2, user3, user4);
    println!("{:?}", white);
}

// parameter shorthand
fn build_user(username: String, email: String, index: u32, active: bool) -> User {
    User {
        username,
        email,
        index,
        active: active,
    }
}
