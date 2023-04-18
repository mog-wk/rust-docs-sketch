
fn main() {
    println!("Ownership\n");

    let s = "strin";
    let sa = s;

    println!("{} {} {}", s, &s, &s as *const &str as usize);
    println!("{} {} {}\n", sa, &sa, &sa as *const &str as usize);

    let s_i = String::from("hello");  // String type
    println!("{} {} {}", s_i, &s_i, &s_i as *const String as usize);
    let s_ii = s_i;  // move, sI no longer valid
    println!("{} {} {}", s_ii, &s_ii, &s_ii as *const String as usize);
    let s_iii = s_ii.clone();  //deep copy
    println!("{} {} {}\n", s_iii, &s_iii, &s_iii as *const String as usize);

    // function ownership
    // Copy trait
    let num: i32 = 12;
    println!("{} {} {}", num, &num, &num as *const i32 as usize);
    let num2 = int_verify(num);
    println!("{} {} {}", num, &num, &num as *const i32 as usize);
    println!("{} {} {}\n", num2, &num2, &num2 as *const i32 as usize);

    // move
    let st = String::from("owner");  // declare st comes into scope
    println!("{} {} {}", st, &st, &st as *const String as usize);
    let sy = string_verify(st);  // move st into string, st no longer valid; sy comes into scope
    println!("{} {} {}\n", sy, &sy, &sy as *const String as usize);

    // references
    let sl = String::from("length");
    println!("{} {} {}", sl, &sl, &sl as *const String as usize);
    let slen = calc_len(&sl);  // calc_len does not take onwership, sl is passed as reference
                               // references are imutable by default
    println!("{} {} {}", sl, &sl, &sl as *const String as usize);
    println!("{} {} {}\n", *slen, slen, &slen as *const &String as usize);

    let sr = String::from("references");  // 
    let rs1 = &sr;
    let rs2 = &sr;
    println!("{} {} {}", sr, rs1, rs2);

    let mut sm = String::from("mut_ref");
    println!("{} {} {}", sm, &sm, &sm as *const String as usize);
    add_to_string(&mut sm);
    println!("{} {} {}", sm, &sm, &sm as *const String as usize);

    let mut s = String::from("lasd");

    let l1 = &mut s;  // can only have one &mut at a time per variable

    *l1 = String::from("123");

    println!("{}", l1);


}


fn add_to_string(st: &mut String) {
    st.push_str("_pog");
}

fn calc_len(st: &String) -> &String {  // borrowing
    st
} 

fn int_verify(n: i32) -> i32 {
    println!("{} {} {}", n, &n, &n as *const i32 as usize);
    n
}

fn string_verify(s: String) -> String {
    println!("{} {} {}", s, &s, &s as *const String as usize);
    s
}

