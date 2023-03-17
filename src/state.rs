use std::env;

pub struct State {
    pub score: u8,
    pub errors: Vec<char>,
    difficulty: i32,
}

impl State {
    pub fn new(mut args: env::Args) -> Result<State, &'static str> {
        let mut difficulty: i32 = 0;

        args.next();

        if let Some(value) = args.next() {
            match value.parse::<i32>() {
                Ok(value) => difficulty = value,
                Err(_) => return Err("Invalid number"),
            }
        }

        Ok(Self {
            score: 0,
            errors: vec![],
            difficulty,
        })
    }

    pub fn valid(&self) -> bool {
        self.errors.len() < 5
    }

    pub fn difficulty(&self) -> i32 {
        self.difficulty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let mut s = State::new(env::args()).unwrap();
        assert_eq!(s.valid(), true);

        s.errors = vec!['a', 'b', 'c', 'd', 'e'];

        assert_eq!(s.valid(), false);
    }
}
