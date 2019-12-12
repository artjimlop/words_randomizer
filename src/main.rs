use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = &args[1].to_lowercase();

    let mut letters_in_word : HashMap<char, String> = HashMap::new();

    for character in word.chars() {
        if !letters_in_word.contains_key(&character) {
            letters_in_word.insert(character, randomize_character(&character));
        }
    }

    let mut final_word = String::from("");
    for character in word.chars() {
        final_word = final_word + letters_in_word.get(&character).unwrap();
    }
    println!("{}", format!("The word `{}` has been randomized to: {}", word, final_word));
}

fn randomize_character(character: &char) -> String {
    let vowels = [ 'a', 'e', 'i', 'o', 'u' ];
    let consonants = [ 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'y', 'z' ];

    let mut rng = thread_rng();
    if vowels.contains(character) {
        return vowels[rng.gen_range(0, vowels.len())].to_string();
    } else if consonants.contains(character) {
        return consonants[rng.gen_range(0, consonants.len())].to_string();
    } else {
        return character.to_string();
    }
}
