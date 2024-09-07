use rand::prelude::IteratorRandom;
use std::fs;
use std::io::{BufRead, BufReader};

pub struct Dictionary {
    words: Vec<String>,
}

impl Dictionary {
    pub fn new(file: &str) -> Self {
        let f = fs::File::open(file).unwrap_or_else(|e| panic!("File not found: {}", e));
        let f = BufReader::new(f);

        Self {
            words: f
                .lines()
                .map(|l| l.expect("Couldn't read line"))
                .filter(|x| x.len() > 3)
                .collect::<Vec<String>>(),
        }
    }

    pub fn random(&self, difficulty: i32) -> String {
        let mut count = 0;

        loop {
            let word = self
                .words
                .iter()
                .choose(&mut rand::thread_rng())
                .expect("File has no lines")
                .to_string();

            if word.len() >= ((difficulty + 3) as usize) {
                return word;
            }

            count += 1;

            if count >= self.words.len() {
                panic!("No words found with that difficulty");
            }

            continue;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "File not found")]
    fn new() {
        Dictionary::new("invalid.txt");
    }

    #[test]
    fn random() {
        let dict = Dictionary::new("one_letter_words.txt");
        let word = dict.random(1);
        assert_eq!(word.len() > 0, true);
    }

    #[test]
    #[should_panic(expected = "No words found with that difficulty")]
    fn random_invalid_difficulty() {
        let dict = Dictionary::new("one_letter_words.txt");
        dict.random(3);
    }
}
