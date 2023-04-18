
#[allow(unused_assignments)]
#[allow(unused_variables)]
fn main() {
    // shadowing
    let x = 10;
    println!("x: {} {}", x, &x as *const i32 as usize);
    let x = 12;
    println!("x: {} {}", x, &x as *const i32 as usize);

    // shadowin with type change
    let x = "1235";   // was i32 is &str
    let x = x.len();  // was &str is usize
    println!("x: {}", x);

    // mutable variable
    let mut y = 12;  // declare mutable varible
    println!("y: {} {}", y, &y as *const i32 as usize);
    y += 4;  // y = 16
    y = 11;  // y = 11
    println!("y: {} {}", y, &y as *const i32 as usize);
    let y = 1;

    // const
    const MAX_VALUE: u32 = 100;
    println!("{} {}", MAX_VALUE, &MAX_VALUE as *const u32 as usize);

    // scalar types
    // integer
    let i: i32 = 32;
    // float 
    let decimal: f64 = 6.4;
    //boolean
    let b: bool = true;
    //char
    let c = '2';

    // compound types
    // tuple
    let tup = (500, 3.14, i);  // declaration
    let tup: (i32, f64, i32) = (500, 3.14, i); // type declaration
    let (x, y, z) = tup;  // deconstruction
    println!("{} {} {}", tup.0, tup.1, tup.2);  // access
    println!("{} {} {}", x, y, z);
    // array
    let arr = [1, 2, 3, 4, 5, 8, 2];
    
    //functions
    println!("{}", add(32, 44));

    // if
    let fact: i32 = factorial(4);
    println!("{}", fact);

    if arr[1] == 4 {
        println!("yes");
    } else if arr[1] == 3 {
        println!("yes");
    } else {
        println!("nop");
    }
    123;

    let i = if arr[1] > 10 {
        tup.0
    } else {
        arr[0]
    }; // variable bind to if statement

    // loops
    
    loop {
        println!("looping");
        break;
    }

    let mut num: u32 = 0;

    while num  < 10 {
        println!("{}", num);
        num+=1;
    }

    for elt in arr.iter() {
        println!("here is element: {}", elt);
    }
    println!("{:?}", arr.iter());

    // exercises
    println!("exercises");
    println!("64 F == {} C", fahrenheit_to_celsius(64));
    println!("72 F == {} C", fahrenheit_to_celsius(72));
    println!("212 F == {} C", fahrenheit_to_celsius(212));
    println!("32 F == {} C", fahrenheit_to_celsius(32));

    println!("fibo  0: {}", fibo(0));
    println!("fibo  1: {}", fibo(1));
    println!("fibo  2: {}", fibo(2));
    println!("fibo  3: {}", fibo(3));
    println!("fibo  4: {}", fibo(4));
    println!("fibo  5: {}", fibo(5));
    println!("fibo  6: {}", fibo(6));
    println!("fibo  7: {}", fibo(7));
    println!("fibo  8: {}", fibo(8));
    println!("fibo  9: {}", fibo(9));
    println!("fibo 12: {}", fibo(12));
    println!("fibo 32: {}", fibo(32));


}

fn fibo(n: u32) -> u32 {
    let mut r: u32 = 1;
    if n > 1 {
        let mut prev = 1;
        let mut c = 1;
        while c < n {
            let tmp = r;
            r = r + prev;  
            prev = tmp;
            c+=1;
        }
    }
    r
}

fn fahrenheit_to_celsius(f_temp: i32) -> i32 {
    (10 * f_temp - 320) / 18
}

#[allow(dead_code)]
fn print_addr(x: usize, y: i32) {
    println!("{} {}", x, &x as *const usize as usize);
    println!("{} {}", y, &y as *const i32 as usize);
}

#[allow(dead_code)]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[allow(dead_code)]
fn is_even(num: i32) -> bool {
    //num % 2 == 0
    if num % 2 == 0 {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
fn factorial(n: i32) -> i32 {
    // if n <= 1 return n else return n * factorial(n-1)
    if n <= 1 {
        return n
    } else {
        return factorial(n - 1) * n
    }
}


