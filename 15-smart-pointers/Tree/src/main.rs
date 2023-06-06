use std::rc::{ Rc, Weak };
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32, parent: RefCell<Weak<Node>>, children: RefCell<Vec<Rc<Node>>>) -> Rc<Node> {
        Rc::new(Self {value, parent, children})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create_node() {
        let l1 = Node::new(3, RefCell::new(vec![]));
        println!(" leaf: strong => {}, weak {}", Rc::strong_count(&l1), Rc::weak_count(&l1));
        {
            let b1 = Node::new(5, RefCell::new(vec![Rc::clone(&l1)]));
            println!(" branch: strong => {}, weak {}", Rc::strong_count(&b1), Rc::weak_count(&b1));


        };

        println!("{:?} {:?}", l1, b1);
    }
}

fn main() {
    let l1 = Node::new(3, RefCell::new(Weak::new()), RefCell::new(vec![]));
    println!(" leaf => strong {}, weak {}", Rc::strong_count(&l1), Rc::weak_count(&l1));
    {
        let b1 = Node::new(5, RefCell::new(Weak::new()), RefCell::new(vec![Rc::clone(&l1)]));
        *l1.parent.borrow_mut() = Rc::downgrade(&b1);
        println!(" leaf => strong {}, weak {}", Rc::strong_count(&l1), Rc::weak_count(&l1));
        println!(" branch => strong {}, weak {}", Rc::strong_count(&b1), Rc::weak_count(&b1));
    };
    println!(" leaf => strong {}, weak {}", Rc::strong_count(&l1), Rc::weak_count(&l1));

}
