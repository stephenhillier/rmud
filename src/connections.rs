pub trait Sender {
    fn send(&self, text: &str);
}

pub struct Connection {}

impl Connection {
    pub fn new() -> Box<dyn Sender> {
        Box::new(Self{})
    }
}

impl Sender for Connection {
    fn send(&self, text: &str) {
        println!("sending {}", text);
    }
}

