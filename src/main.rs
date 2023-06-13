// use wordle_rs::compare::compare;

use wordle_rs::{next_guess::get_next_guess, word_set::WordSet, wordlist::WORD_LIST};

fn main() {
    let mut remaining_words = WordSet::new();
    for word in WORD_LIST.iter().copied() {
        remaining_words.insert(word);
    }
    let first_guess = get_next_guess(&remaining_words);
    println!("First guess: {first_guess}");
}
