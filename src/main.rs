mod dictionary;
mod letter;
mod state;
mod word;

use crate::dictionary::Dictionary;
use crate::state::State;
use crate::word::Word;
use std::io;

fn main() -> Result<(), &'static str> {
    let dict = Dictionary::new();
    let mut word = Word::from(&dict.random());
    let mut state = State::new();

    loop {
        render(&word, &state);

        let letter = ask_for_letter()?;

        if !word.reveal_letter(letter) {
            state.errors += 1;

            if !state.valid() {
                word.reveal();
                render(&word, &state);
                return Ok(());
            }
        } else {
            if word.remaining() == 0 {
                state.score += 1;
                state.errors = 0;
                word = Word::from(&dict.random());
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
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Score: {}", &state.score);
    println!("Attempts: {}/5\n", &state.errors);
    println!("{word}\n");
}
