mod dictionary;
mod letter;
mod state;
mod word;

use crate::dictionary::Dictionary;
use crate::state::State;
use crate::word::Word;
use std::io;

fn main() -> Result<(), &'static str> {
    let dict = Dictionary::new("words.txt");
    let mut state = State::new();
    let mut word = Word::from(&dict.random());
    word.reveal_rand();
    word.reveal_rand();

    loop {
        render(&word, &state);

        match ask_for_letter() {
            Err(err) => println!("{}", err),
            Ok(c) => {
                if !word.reveal_letter(c) {
                    state.errors.push(c);

                    if !state.valid() {
                        word.reveal();
                        render(&word, &state);
                        return Ok(());
                    }
                } else {
                    if word.remaining() == 0 {
                        state.score += 1;
                        state.errors = vec![];
                        word = Word::from(&dict.random());
                        word.reveal_rand();
                        word.reveal_rand();
                    }
                }
            }
        }
    }
}

pub fn ask_for_letter() -> Result<char, &'static str> {
    println!("Enter letter:");

    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        return Err("IO Error");
    }

    let input: String = input.trim().parse().unwrap();

    if input.len() != 1 {
        return Err("Invalid input length");
    }

    Ok(input.chars().next().unwrap())
}

pub fn render(word: &Word, state: &State) {
    let mut used = String::from("");

    for c in &state.errors {
        if let Some(l) = c.to_uppercase().next() {
            used = format!("{used}{l} ");
        }
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Score: {}", &state.score);
    println!("Letters: {}", used);
    println!("Attempts: {}/5\n", &state.errors.len());
    println!("{word}\n");
}
