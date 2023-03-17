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

            continue;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn new() {
        Dictionary::new("invalid.txt");
    }
}
