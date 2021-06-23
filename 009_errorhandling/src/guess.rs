#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 || value > 100 {
            panic!("Invalid value!");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
