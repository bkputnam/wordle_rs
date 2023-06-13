use crate::word_set::Word;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CompareValue {
    NotUsed,
    WrongLocation,
    RightLocation,
}

impl CompareValue {
    fn from_char(c: char) -> CompareValue {
        match c {
            '_' => CompareValue::NotUsed,
            '?' => CompareValue::WrongLocation,
            '.' => CompareValue::RightLocation,
            _ => panic!("Unexpected CompareResult string: {c}"),
        }
    }

    fn to_char(self: &Self) -> char {
        match self {
            CompareValue::NotUsed => '_',
            CompareValue::WrongLocation => '?',
            CompareValue::RightLocation => '.',
        }
    }
}

impl TryFrom<u8> for CompareValue {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            v if v == CompareValue::NotUsed as u8 => Ok(CompareValue::NotUsed),
            v if v == CompareValue::WrongLocation as u8 => Ok(CompareValue::WrongLocation),
            v if v == CompareValue::RightLocation as u8 => Ok(CompareValue::RightLocation),
            _ => Err(()),
        }
    }
}

type CompareValues = [CompareValue; 5];

pub struct CompareResult {
    guess: String,
    values: CompareValues,
}

impl CompareResult {
    pub fn from_string(guess: String, str: String) -> CompareResult {
        let mut chars = str.chars();
        CompareResult {
            guess,
            values: [
                CompareValue::from_char(chars.next().unwrap()),
                CompareValue::from_char(chars.next().unwrap()),
                CompareValue::from_char(chars.next().unwrap()),
                CompareValue::from_char(chars.next().unwrap()),
                CompareValue::from_char(chars.next().unwrap()),
            ],
        }
    }

    pub fn static_value_num(values: &CompareValues) -> u8 {
        let mut result = 0;
        let mut power = 1; // 3**0 == 1
        for i in 0..values.len() {
            let digit = values[i] as u8;
            result += power * digit;
            power *= 3;
        }
        result
    }

    pub fn from_value_num(str: String, value_num: u8) -> CompareResult {
        let mut values: CompareValues = [CompareValue::NotUsed; 5];
        let mut mut_value_num = value_num;
        for i in 0..5 {
            let digit = mut_value_num % 3;
            values[i] = digit.try_into().unwrap();
            mut_value_num = (mut_value_num - digit) / 3;
        }
        CompareResult { guess: str, values }
    }

    pub fn value_str(self: &Self) -> String {
        String::from_iter(self.values.map(|value: CompareValue| value.to_char()))
    }

    pub fn value_num(self: &Self) -> u8 {
        Self::static_value_num(&self.values)
    }

    pub fn is_all_correct(self: &Self) -> bool {
        self.values
            .iter()
            .copied()
            .all(|value: CompareValue| value == CompareValue::RightLocation)
    }

    pub fn to_string(self: &Self) -> String {
        format!("{}|{}", self.guess, self.value_str())
    }
}

pub fn compare_as_num(guess: &Word, actual: &Word) -> Result<u8, String> {
    if guess.len() != actual.len() {
        return Err(format!(
            "Guess was wrong length: expected {}, got {}",
            actual.len(),
            guess.len()
        ));
    }

    let mut values: CompareValues = [CompareValue::NotUsed; 5];
    let mut used_guess_chars = [false; 5];
    let mut used_actual_chars = [false; 5];

    for i in 0..5 {
        if guess.get(i) == actual.get(i) {
            values[i] = CompareValue::RightLocation;
            used_guess_chars[i] = true;
            used_actual_chars[i] = true;
        }
    }

    for guess_index in 0..5 {
        if used_guess_chars[guess_index] {
            continue;
        }
        for actual_index in 0..5 {
            if used_actual_chars[actual_index] {
                continue;
            }
            if guess.get(guess_index) == actual.get(actual_index) {
                values[guess_index] = CompareValue::WrongLocation;
                used_guess_chars[guess_index] = true;
                used_actual_chars[actual_index] = true;
            }
        }
    }
    Ok(CompareResult::static_value_num(&values))
}

pub fn compare(guess: String, actual: String) -> Result<CompareResult, String> {
    match compare_as_num(&Word::from_str(&guess), &Word::from_string(actual)) {
        Ok(value_num) => Ok(CompareResult::from_value_num(guess, value_num)),
        Err(err) => Err(err),
    }
}

#[test]
fn it_should_compute_value_num_correctly() {
    let compare = CompareResult::from_string(String::from("hello"), String::from("_____"));
    assert_eq!(compare.value_num(), 0);
}

#[test]
fn it_should_compute_value_num_correctly_2() {
    let compare = CompareResult::from_string(String::from("hello"), String::from("__?._"));
    assert_eq!(compare.value_num(), (1 * 9) + (2 * 27));
}

#[test]
fn it_should_handle_double_letters() {
    let result = compare(String::from("arbor"), String::from("opera"));
    assert_eq!(
        result.unwrap().values,
        [
            CompareValue::WrongLocation,
            CompareValue::WrongLocation,
            CompareValue::NotUsed,
            CompareValue::WrongLocation,
            CompareValue::NotUsed,
        ]
    );
}

#[test]
fn test_boing_noise() {
    let result = compare(String::from("boing"), String::from("noise"));
    assert_eq!(
        result.unwrap().values,
        [
            CompareValue::NotUsed,
            CompareValue::RightLocation,
            CompareValue::RightLocation,
            CompareValue::WrongLocation,
            CompareValue::NotUsed,
        ]
    );
}

#[test]
fn test_baaaa_aaaac() {
    let result = compare(String::from("baaaa"), String::from("aaaac"));
    assert_eq!(
        result.unwrap().values,
        [
            CompareValue::NotUsed,
            CompareValue::RightLocation,
            CompareValue::RightLocation,
            CompareValue::RightLocation,
            CompareValue::WrongLocation,
        ]
    );
}

#[test]
fn test_aaaab_caaaa() {
    let result = compare(String::from("aaaab"), String::from("caaaa"));
    assert_eq!(
        result.unwrap().values,
        [
            CompareValue::WrongLocation,
            CompareValue::RightLocation,
            CompareValue::RightLocation,
            CompareValue::RightLocation,
            CompareValue::NotUsed,
        ]
    );
}

#[test]
fn test_lares_water() {
    let result = compare(String::from("lares"), String::from("water"));
    assert_eq!(
        result.unwrap().values,
        [
            CompareValue::NotUsed,
            CompareValue::RightLocation,
            CompareValue::WrongLocation,
            CompareValue::RightLocation,
            CompareValue::NotUsed,
        ]
    );
}

#[test]
fn test_kydst_water() {
    let result = compare(String::from("kydst"), String::from("water"));
    assert_eq!(
        result.unwrap().values,
        [
            CompareValue::NotUsed,
            CompareValue::NotUsed,
            CompareValue::NotUsed,
            CompareValue::NotUsed,
            CompareValue::WrongLocation,
        ]
    );
}

#[test]
fn test_rater_water() {
    let result = compare(String::from("rater"), String::from("water"));
    assert_eq!(
        result.unwrap().values,
        [
            CompareValue::NotUsed,
            CompareValue::RightLocation,
            CompareValue::RightLocation,
            CompareValue::RightLocation,
            CompareValue::RightLocation,
        ]
    );
}
