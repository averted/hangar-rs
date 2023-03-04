use crate::letter::Letter;
use rand::Rng;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Word {
    letters: Vec<Letter>,
}

impl Word {
    pub fn from(s: &str) -> Self {
        let mut letters = vec![];
        let rng = rand::thread_rng().gen_range(0..s.chars().count());

        for letter in s.chars() {
            letters.push(Letter::new(letter));
        }

        let l = letters[rng];
        let arr = letters.iter_mut().filter(|x| x.is(l.letter));

        for letter in arr {
            letter.reveal();
        }

        Self { letters }
    }

    pub fn reveal(&mut self) {
        let arr = self.letters.iter_mut();

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
