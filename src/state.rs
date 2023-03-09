pub struct State {
    pub score: u8,
    pub errors: Vec<char>,
}

impl State {
    pub fn new() -> Self {
        Self {
            score: 0,
            errors: vec![],
        }
    }

    pub fn valid(&self) -> bool {
        self.errors.len() < 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let mut s = State::new();
        assert_eq!(s.valid(), true);

        s.errors = vec!['a', 'b', 'c', 'd', 'e'];

        assert_eq!(s.valid(), false);
    }
}
