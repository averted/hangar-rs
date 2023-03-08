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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let mut s = State::new();
        assert_eq!(s.valid(), true);

        s.errors = 5;

        assert_eq!(s.valid(), false);
    }
}
