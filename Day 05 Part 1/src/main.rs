use std::fs;
use std::io::prelude::*;

fn main() {
    // Read input
    let args: Vec<String> = std::env::args().collect();
    let input_filename = args.get(1).expect("No input file given!");
    let file_contents = get_file_contents(&input_filename);

    let mut offsets: Vec<i32> = file_contents.lines()
                                             .map(|x| x.parse::<i32>().unwrap())
                                             .collect();
    let mut position: i32 = 0;
    let mut num_steps = 0;
    loop {
        if position < 0 || position >= offsets.len() as i32 {
            println!("Number of steps: {}", num_steps);
            break;
        }

        let current_offset = offsets[position as usize];
        offsets[position as usize] += 1;
        position += current_offset;
        num_steps += 1;
    }
}

fn get_file_contents(filename: &String) -> String {
    let mut file = fs::File::open(filename).expect("Could not open file!");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file!");
    file_contents = file_contents.trim().to_string();
    file_contents
}
