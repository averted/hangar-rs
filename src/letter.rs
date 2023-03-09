use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Copy, Clone)]
pub struct Letter {
    value: char,
    revealed: bool,
}

impl Letter {
    pub fn new(letter: char) -> Self {
        let mut arr = letter.to_uppercase();
        let mut letter: char = 'X';

        if let Some(l) = arr.next() {
            letter = l;
        }

        Self {
            value: letter,
            revealed: false,
        }
    }

    pub fn reveal(&mut self) {
        self.revealed = true
    }

    pub fn is(&self, letter: char) -> bool {
        let mut arr = letter.to_uppercase();

        match arr.next() {
            Some(letter) => self.value == letter,
            None => false,
        }
    }

    pub fn value(&self) -> char {
        self.value
    }

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", if self.revealed { self.value } else { '_' })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let l = Letter::new('l');
        assert_eq!(l.value, 'L');
        assert_eq!(l.revealed, false);
        assert_eq!(l.to_string(), String::from('_'));
    }

    #[test]
    fn reveal() {
        let mut l = Letter::new('l');
        assert_eq!(l.revealed, false);
        l.reveal();
        assert_eq!(l.revealed, true);
    }

    #[test]
    fn is() {
        let l = Letter::new('l');
        assert_eq!(l.is('l'), true);
        assert_eq!(l.is('L'), true);
        assert_eq!(l.is('c'), false);
        assert_eq!(l.is('_'), false);
    }
}
