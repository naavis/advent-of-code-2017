use std::fs;
use std::io::prelude::*;

fn main() {
    // Read input file
    let args: Vec<String> = std::env::args().collect();
    let input_filename = args.get(1).expect("No input file given!");
    let file_contents = get_file_contents(&input_filename);

    // Initialize memory banks
    let mut banks: Vec<u64> = file_contents.trim()
                                            .split('\t')
                                            .map(|x| x.parse::<u64>().unwrap())
                                            .collect();

    let num_banks = banks.len();
    let mut seen_combinations: Vec<Vec<u64>> = Vec::new();
    let mut iteration = 0;
    loop {
        // Check if memory block combination has already been seen
        if seen_combinations.iter().position(|ref r| **r == banks).is_some() {
            println!("Iterations: {}", iteration);
            break;
        }
        seen_combinations.push(banks.clone());

        // Redistribute memory blocks
        let max_idx = argmax(&banks);
        let max_blocks = banks[max_idx];
        banks[max_idx] = 0;
        for i in 1..max_blocks + 1 {
            banks[(max_idx + i as usize) % num_banks] += 1;
        }
        iteration += 1;
    }
}

fn get_file_contents(filename: &String) -> String {
    let mut file = fs::File::open(filename).expect("Could not open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file!");
    contents = contents.trim().to_string();
    contents
}

fn argmax(data: &Vec<u64>) -> usize {
    let mut max_idx: usize = 0;
    let mut max_value: u64 = 0;
    for (idx, value) in data.iter().enumerate() {
        if *value > max_value {
            max_value = *value;
            max_idx = idx;
        }
    }
    max_idx
}
