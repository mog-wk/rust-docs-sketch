use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut cur = self::Cons;
        Ok(())

        //while cur != List::Nil {
            //write!(f, "{} ", self.Cons.0);
            //cur = self.Cons.1;
        //}
        //write!(f, "{}", self.Cons.0)
    }
}

struct myBox<T>(T);

impl<T> myBox<T> {
    fn new(x: T) -> myBox<T> {
        myBox(x)
    }
}

impl<T> Deref for myBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

type CSP = CustomSmartPointer;

impl CSP {
    fn new(data: String) -> Self {
        Self { data }
    }
}

impl Drop for CSP {
    fn drop(&mut self) {
        println!("dropping smart pointer \"{}\"", self.data);
    }
}

impl std::fmt::Display for CSP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

use List::{Cons, Nil};

fn main() {
    println!("Hello, world!");

    let b_x = Box::new(5);
    println!("b_x => {}", b_x);
    println!("ref b_x => {}", get_id(*b_x));

    let list_1 = Cons(1, Rc::new(Cons(2, Rc::new(Nil))));

    println!("{}", list_1);


    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Rc count => {}", Rc::strong_count(&a));
    let b = Cons(4, Rc::clone(&a));
    println!("Rc count => {}", Rc::strong_count(&a));
    let c = Cons(2, Rc::clone(&a));
    println!("Rc count => {}", Rc::strong_count(&a));


    let ptr_c = CustomSmartPointer::new(String::from("c => my stuff"));
    let ptr_d = CustomSmartPointer::new(String::from("d => is rotten"));

    drop(ptr_c);
    println!("end of main");
}


fn get_id<T>(val: T) -> usize {
    &val as *const T as usize
}
