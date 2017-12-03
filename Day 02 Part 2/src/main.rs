use std::fs;
use std::io::prelude::*;

fn main() {
    // Read input
    let args: Vec<String> = std::env::args().collect();
    let input_filename = args.get(1).expect("No input file given!");
    let file_contents = get_file_contents(&input_filename);

    let mut checksum = 0;
    for row in file_contents.split('\n') {
        let numbers: Vec<u32> = row.trim()
                                   .split('\t')
                                   .map(|n| n.parse::<u32>().unwrap())
                                   .collect();

        let mut solution_found = false;
        for a in numbers.iter() {
            for b in numbers.iter() {
                if a <= b {
                    continue;
                }
                if a % b == 0 {
                    checksum += a / b;
                    solution_found = true;
                    break;
                }
            }
            if solution_found {
                break;
            }
        }
    }

    println!("The result is: {}", checksum);
}

fn get_file_contents(filename: &String) -> String {
    let mut file = fs::File::open(filename).expect("Could not open file!");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file!");
    file_contents = file_contents.trim().to_string();
    file_contents
}
