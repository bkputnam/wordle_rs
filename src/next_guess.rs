use std::time::Instant;

use crate::{
    compare::compare_as_num,
    word_set::{Word, WordSet},
    wordlist::WORD_LIST,
};

fn get_expected_remaining_words(word_list: &WordSet, word: &Word) -> f64 {
    let mut groups = [0; (3 as usize).pow(5)];
    for actual_word in word_list.iter() {
        let result = compare_as_num(word, actual_word).unwrap() as usize;
        groups[result] += 1;
    }

    let mut sum = 0;
    let mut num_words = 0;
    for size in groups {
        sum += size * size;
        num_words += size;
    }

    (sum as f64) / (num_words as f64)
}

pub fn get_next_guess(remaining_words: &WordSet) -> &Word {
    match remaining_words.len() {
        0 => panic!("Cannot create a guess with an empty word list."),
        1 => {
            return remaining_words.iter().next().unwrap();
        }
        _ => {}
    }

    let mut min_expected_remaining_words = f64::INFINITY;
    let mut next_guess: &Word = WORD_LIST.iter().next().unwrap();

    let timer = Instant::now();

    for word in WORD_LIST.iter() {
        let is_plausible_word = remaining_words.contains(word);
        let expected_remaining_words = get_expected_remaining_words(remaining_words, word);
        if expected_remaining_words < min_expected_remaining_words
            || (expected_remaining_words == min_expected_remaining_words && is_plausible_word)
        {
            min_expected_remaining_words = expected_remaining_words;
            next_guess = word;
        }
    }

    println!(
        "Picked next guess {next_guess} in {} sec",
        timer.elapsed().as_secs()
    );
    println!("Expected remaining words after {next_guess}: {min_expected_remaining_words}");
    next_guess
}

#[test]
fn test_smoke() {
    println!("{}", WORD_LIST.get(0).unwrap());
}
