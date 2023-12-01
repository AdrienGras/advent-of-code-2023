use std::fs::read_to_string;

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
}
