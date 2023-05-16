

struct CompostPoint<T, U> {
    x: T,
    y: U,
}

impl<T: std::ops::AddAssign, U: std::ops::AddAssign> CompostPoint<T, U> {
    fn new(x: T, y: U) -> Self {
        Self {x, y}
    }

    // add result for err check
    fn translate(&mut self, n: T, m: U) {
        self.x += n;
        self.y += m;
    }
}

