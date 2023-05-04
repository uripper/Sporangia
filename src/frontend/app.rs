pub struct App {
    value: i64,
}

impl App {
    pub fn new(integer: i64) -> Self {
        App { value: integer }
    }
    pub fn run(self) {
        println!("Hello, world! {}", self.value);
    }
}
