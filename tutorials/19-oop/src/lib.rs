pub mod blog;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button: {} ({}x{})",
            self.label, self.width, self.height
        );
    }
}