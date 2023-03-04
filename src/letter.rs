use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Copy, Clone)]
pub struct Letter {
    pub letter: char,
    pub revealed: bool,
}

impl Letter {
    pub fn new(letter: char) -> Self {
        let mut arr = letter.to_uppercase();
        let mut letter: char = 'X';

        if let Some(l) = arr.next() {
            letter = l;
        }

        Self {
            letter,
            revealed: false,
        }
    }

    pub fn reveal(&mut self) {
        self.revealed = true
    }

    pub fn is(&self, letter: char) -> bool {
        let mut arr = letter.to_uppercase();

        match arr.next() {
            Some(letter) => self.letter == letter,
            None => false,
        }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", if self.revealed { self.letter } else { '_' })
    }
}
