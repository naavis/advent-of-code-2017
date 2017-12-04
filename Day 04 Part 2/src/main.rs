use std::fs;
use std::io::prelude::*;

fn main() {
    // Read input
    let args: Vec<String> = std::env::args().collect();
    let input_filename = args.get(1).expect("No input file given!");
    let file_contents = get_file_contents(&input_filename);

    // Count valid passphrases
    let mut correct_count = 0;
    for password in file_contents.lines() {
        let mut valid = true;
        let words: Vec<&str> = password.split(' ').collect();
        let mut used_words: Vec<String> = Vec::new();
        for current_word in words {
            // Normalize word by sorting characters before comparison
            let mut chars_in_word: Vec<char> = current_word.chars().collect();
            chars_in_word.sort_by(|a, b| b.cmp(a));
            let normalized_word: String = chars_in_word.into_iter().collect();
            if used_words.contains(&normalized_word) {
                valid = false;
            }
            used_words.push(normalized_word);
        }
        if valid {
            correct_count += 1;
        }
    }
    println!("Number of valid passphrases: {}", correct_count);
}

fn get_file_contents(filename: &String) -> String {
    let mut file = fs::File::open(filename).expect("Could not open file!");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file!");
    file_contents = file_contents.trim().to_string();
    file_contents
}
