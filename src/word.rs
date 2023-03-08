use crate::letter::Letter;
use rand::Rng;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Word {
    letters: Vec<Letter>,
}

impl Word {
    pub fn from(s: &str) -> Self {
        let mut letters = vec![];

        for letter in s.chars() {
            letters.push(Letter::new(letter));
        }

        Self { letters }
    }

    pub fn reveal(&mut self) {
        let arr = self.letters.iter_mut();

        for letter in arr {
            letter.reveal();
        }
    }

    pub fn reveal_rand(&mut self) {
        let rng = rand::thread_rng().gen_range(0..self.letters.len());
        let l = self.letters[rng];
        let arr = self.letters.iter_mut().filter(|x| x.is(l.value));

        for letter in arr {
            letter.reveal();
        }
    }

    pub fn reveal_letter(&mut self, letter: char) -> bool {
        let size = self.letters.iter().filter(|x| x.is(letter)).count();
        let arr = self.letters.iter_mut().filter(|x| x.is(letter));

        for letter in arr {
            letter.reveal();
        }

        size > 0
    }

    pub fn remaining(&self) -> usize {
        self.letters.iter().filter(|x| !x.revealed).count()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let letters: Vec<String> = self.letters.iter().map(|x| x.to_string()).collect();
        write!(f, "{:?}", letters.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let w = Word::from("testing");
        assert_eq!(w.to_string(), "\"_ _ _ _ _ _ _\"");
    }

    #[test]
    fn reveal() {
        let mut w = Word::from("testing");
        w.reveal();
        assert_eq!(w.to_string(), "\"T E S T I N G\"");
    }

    #[test]
    fn reveal_rand() {
        let s = "testing";
        let mut w = Word::from(s);
        w.reveal_rand();
        assert!(w.remaining() < s.len());
    }

    #[test]
    fn reveal_letter() {
        let mut w = Word::from("testing");
        w.reveal_letter('t');
        assert_eq!(w.to_string(), "\"T _ _ T _ _ _\"");

        let mut w2 = Word::from("rust");
        w2.reveal_letter('u');
        assert_eq!(w2.to_string(), "\"_ U _ _\"");
    }

    #[test]
    fn remaining() {
        let mut w = Word::from("testing");
        w.reveal_letter('t');
        assert_eq!(w.remaining(), 5);

        let mut w2 = Word::from("rust");
        w2.reveal_letter('u');
        assert_eq!(w2.remaining(), 3);
    }
}
