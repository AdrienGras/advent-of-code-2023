use std::{fs::read_to_string, collections::HashMap};

const FILEPATH: &str = "./input.txt";
const TEST_FILE_PART1: &str = "./input_ex_1.txt";
const TEST_FILE_PART2: &str = "./input_ex_2.txt";

fn part_1(file_path: &str) -> i32 {
    let mut values: Vec<i32> = Vec::new();

    // read the file and split it into lines
    for line in read_to_string(file_path).unwrap().lines() {
        let mut line_values: Vec<i32> = Vec::new();

        // iterate over chars of line
        // if char is a digit, push it to the vector
        for c in line.chars() {
            if c.is_ascii_digit() {
                line_values.push(c.to_digit(10).unwrap() as i32);
            }
        }

        if line_values.is_empty() {
            continue;
        }

        // get the first and last digit of the vector
        let first_digit = line_values[0];
        let last_digit = line_values[line_values.len() - 1];

        // assemble the two digits as one number and push it to the vector
        let number = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
        values.push(number);
    }

    // sum all values of the vector "values"
    let sum: i32 = values.iter().sum();
    
    sum
}

fn part_2(file_path: &str) -> i32 {
    let mut values: Vec<i32> = Vec::new();

    // read the file and split it into lines
    for line in read_to_string(file_path).unwrap().lines() {

        let mut temp_line = String::from(line);
        let mut line_values: Vec<i32> = Vec::new();

        // iterate over chars of line while the line is not empty
        // if char is a digit, push it to the vector
        // if the line starts with a number, push the number to the vector
        // remove the first char of the line
        while !temp_line.is_empty() {
            let c = temp_line.chars().next().unwrap();

            if c.is_ascii_digit() {
                line_values.push(c.to_digit(10).unwrap() as i32);
            } else if temp_line.starts_with("one") {
                line_values.push(1);
            } else if temp_line.starts_with("two") {
                line_values.push(2);
            } else if temp_line.starts_with("three") {
                line_values.push(3);
            } else if temp_line.starts_with("four") {
                line_values.push(4);
            } else if temp_line.starts_with("five") {
                line_values.push(5);
            } else if temp_line.starts_with("six") {
                line_values.push(6);
            } else if temp_line.starts_with("seven") {
                line_values.push(7);
            } else if temp_line.starts_with("eight") {
                line_values.push(8);
            } else if temp_line.starts_with("nine") {
                line_values.push(9);
            }

            temp_line.remove(0);
        }

        if line_values.is_empty() {
            continue;
        }

        // get the first and last digit of the vector
        let first_digit = line_values[0];
        let last_digit = line_values[line_values.len() - 1];

        // assemble the two digits as one number and push it to the vector
        let number = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
        values.push(number);
    }

    // sum all values of the vector "values"
    let sum: i32 = values.iter().sum();

    sum
}

fn generate_digit_map() -> HashMap<i32, String> {
    let mut digit_map: HashMap<i32, String> = HashMap::new();

    digit_map.insert(1, String::from("one"));
    digit_map.insert(2, String::from("two"));
    digit_map.insert(3, String::from("three"));
    digit_map.insert(4, String::from("four"));
    digit_map.insert(5, String::from("five"));
    digit_map.insert(6, String::from("six"));
    digit_map.insert(7, String::from("seven"));
    digit_map.insert(8, String::from("eight"));
    digit_map.insert(9, String::from("nine"));

    digit_map
}

fn part_2_proper(file: &str) -> i32 {
    let lines = read_to_string(file).unwrap();

    let digit_map = generate_digit_map();

    let sum = lines
    .lines()
    .collect::<Vec<&str>>()
    .iter()
    .fold(0, |acc, line| {
        let line = line.to_string();

        let line_values = (0..line.len()).filter_map(|index| {
            let temp_line = &line[index..];

            let first_char: char = temp_line.chars().next().unwrap();

            if first_char.is_ascii_digit() {
                Some(first_char.to_digit(10).unwrap() as i32)
            } else {
                for (digit, word) in digit_map.iter() {
                    if temp_line.starts_with(word) {
                        return Some(*digit);
                    }
                }

                None
            }
        })
        .collect::<Vec<i32>>();

        if line_values.is_empty() {
            return acc;
        }

        let first_digit = line_values[0];
        let last_digit = line_values[line_values.len() - 1];

        let number = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
        acc + number
    });

    sum
}

fn main() {
    println!("=== Advent of Code 2023 - Day 1 ===\n");

    println!("--- Part 1 ---");

    println!("PART1: Test file");
    let result = part_1(TEST_FILE_PART1);
    let expected = 142;
    println!("Test result: {} (expected: {})", result, expected);

    if result == expected {
        println!("Test successful!");
    } else {
        panic!("Test failed!");
    }

    println!("PART1: Input file");
    let result = part_1(FILEPATH);
    println!("Result: {}", result);

    println!("\n--- Part 2 ---");

    println!("PART2: Test file");
    let result = part_2(TEST_FILE_PART2);
    let expected = 281;
    println!("Test result: {} (expected: {})", result, expected);

    if result == expected {
        println!("Test successful!");
    } else {
        panic!("Test failed!");
    }

    println!("PART2: Input file");
    let result = part_2(FILEPATH);
    println!("Result: {}", result);

    println!("\n--- Part 2 proper way ---");

    println!("PART2: Test file");
    let result = part_2_proper(TEST_FILE_PART2);
    let expected = 281;
    println!("Test result: {} (expected: {})", result, expected);

    if result == expected {
        println!("Test successful!");
    } else {
        panic!("Test failed!");
    }

    println!("PART2: Input file");
    let result = part_2_proper(FILEPATH);
    println!("Result: {}", result);
}
