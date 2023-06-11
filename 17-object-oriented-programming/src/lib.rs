use std::fmt::Display;


pub struct AvarageCollection {
    // struct for list abstraction and avarege calculation
    list: Vec<i32>,
    avarege: f64,
}

impl AvarageCollection {
    pub fn new(list: Vec<i32>) -> Self {
        // creates a new instance of Self
        let s: i32 = list.iter().sum();
        let len = list.len(); 
        Self {list, avarege: s as f64 / len as f64} 
    }
    pub fn add(&mut self, val: i32) {
        // add element in the end of self.list
        self.list.push(val);
        self.update_avarege();
    }
    pub fn pop(&mut self) -> Option<i32> {
        // remove last element from self.list
        match self.list.pop() {
            Some(val) => {
                self.update_avarege();
                Some(val)
            },
            None => None,
        }
    }

    pub fn avarage(&self) -> f64 { self.avarege }

    fn update_avarege(&mut self) {
        // update self.avarege call always we add/removing/mutating self.list
        self.avarege = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
    }
}

impl Display for AvarageCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {}", self.list, self.avarege)
    }
}


