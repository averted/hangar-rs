pub struct State {
    pub score: u8,
    pub errors: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            score: 0,
            errors: 0,
        }
    }

    pub fn valid(&self) -> bool {
        self.errors < 5
    }
}
