use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let file = File::open("words.txt").expect("unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line.expect("invalid line");
        let cleanword = get_hex_sanitised_and_substituted_word(word.as_str());
        if cleanword.chars().count() == 3 || cleanword.chars().count() == 6 {
            if cleanword.chars().all(|c| is_valid_hex_char(c)) {
                println!("{}: #{}", &word, cleanword);
            }
        }    
    }
}

fn get_hex_sanitised_and_substituted_word(word: &str) -> String {
    let uppercase = word.to_uppercase();
    let cleanword : String = uppercase.chars().filter(|c| c.is_alphanumeric()).collect();
    let substituted = cleanword.chars()
        .map(|x| match x { 
            'O' => '0', 
            'S' => '5',
            'I' => '1',
            'L' => '1',
            'T' => '7',
            'Q' => '9',
            _ => x
        }).collect();
    return substituted
}

fn is_valid_hex_char(c: char) -> bool {
    return match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' => true,
        _=> false
    }
}