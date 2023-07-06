struct RGB {
    R: u8,
    G: u8,
    B: u8,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Color: ");
        match self {
            Color::Red => write!(f, "Red"),
            Color::Blue => write!(f, "Blue"),
            _ => write!(f, "Color not found")
        }
    }
}

fn _if_let() {
    let colors = vec![Color::Red, Color::Blue];

    let fav_color: Option<Color> = Some(Color::Blue);
    
    for color in colors {
        if let Some(c) = &fav_color {
            println!("{}", c);
            println!("{:?}", c);
        }
    }
}

fn _while_let() {

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("{:?}", stack);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    println!("{:?}", stack);
}

fn hash_from_vec<T>(vec: &Vec<T>) -> Option<String> where T: std::fmt::Display {
    let mut out: String = String::new();
    for (k, v) in vec.iter().enumerate() {
        out += &format!("{}: {}, ", k, v);
    }
    Some(out)
}


fn get_location(&(x, y): &(i32, i32)) -> (i32, i32) {
    (x, y)
}


fn main() {
    println!("Hello, world!");

    /*
    let v_1: Vec<char> = vec!['a', 'b', 'c', 'd'];
    let out: Option<String> = hash_from_vec(&v_1);

    if let Some(o) = out {
        println!("{}", o);
    }

    let (x, y, z) = (1, 2, 3);

    println!("{} {} {}", x, y, z);

    let (x, y) = get_location(&(32, 12));

    println!("{} {}", x, y);
    let x = |x: i32| -> i32 {
        x + 2
    };

    println!("{}", x(2));

    let y = |&(x, y): &(u32, u32)| -> f64 {
        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    };

    println!("{}", y(&(3, 4)) );
    */

    let x = 5; // inrefutable patten
    if let x = Some(5) {} // refutable patten x can only match type &str
    // let Some(x) = None // Error
    // if let x = 5 //Error x = 5 is an inrefutable patten


    match x {
        1 | 2 => println!("x < 3"),
        3 => println!("x == 3"),
        1..=10 => println!("x 1...5"),
        _ => println!("x not in scope"),
    }

    let c_vec: Vec<char> = vec!['a', 'b', 'e', 'x', 'z', '4', '-', '+', '\t'];
    for c in c_vec {
        let out = match &c {
            'a'..='k' => "early_ASCII",
            'k'..='z' =>  "late_ASCII",
            _ => "dafault",
        };
        println!("{}", out)
    }



}

