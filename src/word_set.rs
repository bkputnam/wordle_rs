// use std::slice::Iter;

// const fn ceil_div(numerator: usize, denominator: usize) -> usize {
//     (numerator + denominator - 1) / denominator
// }
// pub struct WordSet {
//     bits: [u8; ceil_div((26 as usize).pow(5), 8)],
//     words: Vec<String>,
// }

// fn word_to_index(word: &str) -> usize {
//     let mut pow = 0;
//     let mut result: usize = 0;
//     for c in word.chars() {
//         let char_num = (c as usize) - ('a' as usize);
//         result += char_num * (26 as usize).pow(pow);
//         pow += 1;
//     }
//     result
// }

// fn word_to_byte_and_bit_index(word: &str) -> (usize, u8) {
//     let overall_index = word_to_index(word);
//     let byte_index = overall_index / 8;
//     let bit_index: u8 = match overall_index % 8 {
//         0 => 0,
//         1 => 2,
//         2 => 4,
//         3 => 8,
//         4 => 16,
//         5 => 32,
//         6 => 64,
//         7 => 128,
//         _ => panic!("This should be impossible"),
//     };
//     (byte_index, bit_index)
// }

// impl WordSet {
//     pub fn new() -> WordSet {
//         WordSet {
//             bits: [0 as u8; ceil_div((26 as usize).pow(5), 8)],
//             words: vec![],
//         }
//     }

//     pub fn insert(self: &mut Self, str: String) {
//         // self.bits[self.word_to_index(&str)] = true;
//         let (byte_index, bit_index) = word_to_byte_and_bit_index(&str);
//         let byte = &mut self.bits[byte_index];
//         *byte = *byte | bit_index;
//         self.words.push(str);
//     }

//     pub fn contains(self: &Self, str: &str) -> bool {
//         let (byte_index, bit_index) = word_to_byte_and_bit_index(&str);
//         self.bits[byte_index] & bit_index != 0
//     }

//     pub fn len(self: &Self) -> usize {
//         self.words.len()
//     }

//     pub fn iter(self: &Self) -> Iter<String> {
//         self.words.iter()
//     }
// }

use std::slice::Iter;

pub struct WordSet {
    words: Vec<String>,
}

impl WordSet {
    pub fn new() -> WordSet {
        WordSet { words: vec![] }
    }

    pub fn insert(self: &mut Self, str: String) {
        match self.words.binary_search(&str) {
            Ok(_) => { /* element already in vector */ }
            Err(pos) => self.words.insert(pos, str),
        };
    }

    pub fn contains(self: &Self, str: &str) -> bool {
        match self
            .words
            .binary_search_by(|word: &String| word.as_str().cmp(str))
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn len(self: &Self) -> usize {
        self.words.len()
    }

    pub fn iter(self: &Self) -> Iter<String> {
        self.words.iter()
    }
}

// use radix_trie::Trie;
// use radix_trie::TrieCommon;

// pub struct WordSet {
//     words: Trie<String, bool>,
//     len: usize,
// }

// impl WordSet {
//     pub fn new() -> WordSet {
//         WordSet {
//             words: Trie::new(),
//             len: 0,
//         }
//     }

//     pub fn insert(self: &mut Self, str: String) {
//         self.words.insert(str, true);
//         self.len += 1;
//     }

//     pub fn contains(self: &Self, str: &str) -> bool {
//         match self.words.get(str) {
//             Some(_) => true,
//             None => false,
//         }
//     }

//     pub fn len(self: &Self) -> usize {
//         self.len
//     }

//     pub fn iter(self: &Self) -> impl Iterator<Item = &String> {
//         // TrieIter::new(&self.words.node).map(|str, _| str)
//         self.words.iter().map(|(str, _)| str)
//     }
// }

#[test]
fn test_basics() {
    let mut word_set = WordSet::new();
    word_set.insert(String::from("foozz"));
    word_set.insert(String::from("barzz"));
    word_set.insert(String::from("bazzz"));

    assert_eq!(word_set.contains(&String::from("foozz")), true);
    assert_eq!(word_set.contains(&String::from("barzz")), true);
    assert_eq!(word_set.contains(&String::from("bazzz")), true);
    assert_eq!(word_set.contains(&String::from("batzz")), false);
    assert_eq!(word_set.contains(&String::from("bagzz")), false);
}

#[test]
fn test_iter_doesnt_consume() {
    let mut word_set = WordSet::new();
    word_set.insert(String::from("foozz"));
    word_set.insert(String::from("barzz"));
    word_set.insert(String::from("bazzz"));

    let iter1: Vec<&String> = word_set.iter().collect();
    assert_eq!(iter1.contains(&&String::from("foozz")), true);
    assert_eq!(iter1.contains(&&String::from("barzz")), true);
    assert_eq!(iter1.contains(&&String::from("bazzz")), true);

    let iter2: Vec<&String> = word_set.iter().collect();
    assert_eq!(iter2.contains(&&String::from("foozz")), true);
    assert_eq!(iter2.contains(&&String::from("barzz")), true);
    assert_eq!(iter2.contains(&&String::from("bazzz")), true);
}
