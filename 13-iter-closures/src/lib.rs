use std::collections::HashMap;

//struct Cacher<T: Fn(u32) -> u32> {
pub struct Cacher<T> where T: Fn(u32) -> u32 {
    calc: T,
    map: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    pub fn new(calc: T) -> Cacher<T> {
        Cacher {
            calc,
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        for (key, v) in &self.map {
            if key == &arg {
                return *v
            }
        }
        let v = (self.calc)(arg);
        self.map.insert(arg, v);
        v
    }
     pub fn display(&self) -> &HashMap<u32, u32> {
         &self.map
     }

}


#[cfg(test)]
mod tests {
    use crate::Cacher;
    #[test]
    fn test_cacher_value() {
        let mut c1 = Cacher::new(|n| n*2);

        assert_eq!(c1.value(3), 6);
        assert_eq!(c1.value(4), 8);
        assert_eq!(c1.value(2), 4);
    }
}

