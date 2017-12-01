use std::fs;
use std::io::prelude::*;

fn main() {
    // Read input
    let args: Vec<String> = std::env::args().collect();
    let input_filename = args.get(1).expect("No input file given!");
    let mut file = fs::File::open(input_filename).expect("Could not open file!");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file!");
    file_contents = file_contents.trim().to_string();

    let mut numbers: Vec<u8> = Vec::new();
    for c in file_contents.chars() {
        let parsed_digit: u8 = c.to_digit(10).expect("Invalid character in input!") as u8;
        numbers.push(parsed_digit);
    }

    let sum = get_captcha_sum(&numbers);
    println!("Captcha sum is: {}", sum);
}

fn get_captcha_sum(numbers: &Vec<u8>) -> u64 {
    let mut sum: u64 = 0;
    let num_items = numbers.len();
    for (i, number) in numbers.iter().enumerate() {
        let next_index = (i + 1) %  num_items;
        if *number == numbers[next_index] {
            sum += *number as u64;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! captcha_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_captcha_sum(input));
            }
        )*
        }
    }

    captcha_tests! {
        test_01: (&vec![1, 1, 2, 2], 3),
        test_02: (&vec![1, 1, 1, 1], 4),
        test_03: (&vec![1, 2, 3, 4], 0),
        test_04: (&vec![9, 1, 2, 1, 2, 1, 2, 9], 9),
    }
}
