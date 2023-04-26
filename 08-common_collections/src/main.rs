use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // Vectors
    let v1: Vec<i32> = Vec::new(); // constructor
    let v2 = vec![1, 2, 3]; // macro
    let mut v3 = vec![4, 5, 6]; // mut vector varies in size but not in type
    
    v3.push(-12);

    println!("{:?} {:?} {:?}", v1, v2, v3);

    // #access
    let r1: &i32 = &v2[2]; 
    let r2: Option<&i32> = v2.get(2);
    let r3: &i32 = match v2.get(1) { Some(n) => n, None => panic!() };
    //let r4: &i32 = match v2.get(12) { Some(n) => n, None => panic!("index out of bounds") };
    println!("{}, {:?} {:?}", r1, r2, r3 ); 

    //iterate
    for i in &v2 {
        println!("{} ", i);
    }print!("\n");

    for i in &mut v3 {
        *i *= 2;
        println!("{} |", i);
    }print!("\n");

    //enums

    #[derive(Debug)]
    enum SSC {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let din_vec = vec![SSC::Int(44), SSC::Float(3.14), SSC::Text(String::from("a text sample"))];

    for elt in &din_vec {
        println!("{:?} ", elt);
    }print!("\n");

    // strings

    let s1 = String::new();
    let s2 = "a string sample".to_string();
    let s3 = String::from("another string sample");
    let string_v = vec![String::new(), "a string sample".to_string(), String::from("another string sample"),];

    //format!("{} {}", s2, s3);

    print_str_arr(&string_v);

    // iterate

    for c in s3.chars() {
        print!("{} ", c);
    }
    print!("\n");

    for b in s3.bytes() {
        print!("{} ", b);
    }
    print!("\n");

    // hash maps
    //


    let mut scores = HashMap::new(); // constructor

    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 60);
    scores.insert(String::from("Yellow"), 90);
    scores.insert(s3, 90);

    println!("{:?}", scores);

    let score_log: HashMap<_,_> = v2.iter().zip(string_v.iter()).collect(); // make hash map from a vector of tuples
    println!("{:?}", score_log);

    // access
    println!("{:?} {:?}", scores.get("Red"), scores.get("Blue"));

    //iterate

    for (k, v) in scores {
        println!("{} {:?}", k, v);
    }

    let mp: HashMap<_, _> = word_count("this is a string slice of string");
    let np: HashMap<&str, u32> = word_count("this is a string slice of string");

    println!("{:?}", mp);
    println!("{:?}", np);


}

fn print_str_arr<T: std::fmt::Display>(arr: &Vec<T>) {
    for i in arr {
        print!("\"{}\" ", i);
    } print!("\n");
}

fn word_count(st: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();
    for word in st.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
}
