

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub hright: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button :(");
    }
}


