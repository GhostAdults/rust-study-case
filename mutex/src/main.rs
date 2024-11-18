use std::sync::Mutex;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {
        // drawing
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        // drawing
    }
}

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 23,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            }),
            Box::new(Button {
                width: 20,
                height: 20,
                label: String::from("ok"),
            }),
        ],
    };
    screen.run();
    println!("Hello, world! {:?}", m);
}
