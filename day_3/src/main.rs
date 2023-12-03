use std::collections::HashMap;

use regex::Regex;

const PUZZLE_INPUT: &str = "input.txt";
const EXAMPLE_INPUT_PART_1: &str = "example1.txt";
const EXAMPLE_INPUT_PART_2: &str = "example2.txt";

fn part_1(file_path: &str) -> u32 {
    // loading input
    let input = std::fs::read_to_string(file_path).unwrap();

    // storing limits for rows and columns
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    // storing numbers and symbols
    let mut numbers: Vec<(&str, Position)> = vec![];
    let mut symbols: Vec<(char, Position)> = vec![];

    // storing part numbers for sum
    let mut part_numbers: Vec<u32> = vec![];

    // setting up regex for numbers
    let re = Regex::new(r"\d+").unwrap();

    // iterating over lines
    for (idx, line) in input.lines().enumerate() {
        // foreach number in line
        // we store it and its position
        for m in re.find_iter(line) {
            numbers.push((
                m.as_str(),
                Position(idx as i32, m.start() as i32),
            ));
        }

        // and we store symbols and their position
        for (col, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                symbols.push((ch, Position(idx as i32, col as i32)));
            }
        }
    }

    // for each number we check if it is in the range of a symbol
    for (number, start_position) in numbers.iter() {
        // determining end position of number
        let num_end_pos = Position(start_position.0, start_position.1 + number.len() as i32 - 1);

        // checking if number is in range of a symbol
        for (_, symbol_pos) in symbols.iter() {
            // creating positions of all neighbors
            let neighbors = [
                Position(symbol_pos.0 - 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 - 1, symbol_pos.1),
                Position(symbol_pos.0 - 1, symbol_pos.1 + 1),
                Position(symbol_pos.0, symbol_pos.1 - 1),
                Position(symbol_pos.0, symbol_pos.1 + 1),
                Position(symbol_pos.0 + 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 + 1, symbol_pos.1),
                Position(symbol_pos.0 + 1, symbol_pos.1 + 1),
            ]
            .into_iter()
            .filter(|p| {
                p.0 >= 0 && p.1 >= 0 && p.0 < num_rows as i32 && p.1 < num_cols as i32
            })
            .collect::<Vec<Position>>();

            // checking if number is in range of symbol
            for neighbor in neighbors.iter() {
                if neighbor.0 == start_position.0
                    && neighbor.1 >= start_position.1
                    && neighbor.1 <= num_end_pos.1
                {
                    part_numbers.push(number.parse::<u32>().unwrap());
                    break;
                }
            }
        }
    }
    part_numbers.iter().sum()
}

fn part_2(file_path: &str) -> u32 {
    // loading input
    let input = std::fs::read_to_string(file_path).unwrap();

    // storing limits for rows and columns
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    // storing numbers and gears
    let mut numbers: Vec<(&str, Position)> = vec![];
    let mut gears: Vec<(char, Position)> = vec![];

    // storing gear and neighbor numbers
    let mut gear_table: HashMap<Position, Vec<u32>> = HashMap::new();

    // setting up regex for numbers
    let re = Regex::new(r"\d+").unwrap();

    // iterating over lines
    for (row, line) in input.lines().enumerate() {
        // foreach number in line
        for m in re.find_iter(line) {
            // we store it and its position
            numbers.push((
                m.as_str(),
                Position(row as i32, m.start() as i32),
            ));
        }

        // and we store gears and their position
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                gears.push((ch, Position(row as i32, col as i32)));
            }
        }
    }

    // for each number we check if it is in the range of a gear
    for (num, start_position) in numbers.iter() {
        // determining end position of number
        let num_end_pos = Position(start_position.0, start_position.1 + num.len() as i32 - 1);

        // looping over gears
        for (_, gear_pos) in gears.iter() {

            // creating possible positions of all neighbors
            let neighbors = [
                Position(gear_pos.0 - 1, gear_pos.1 - 1),
                Position(gear_pos.0 - 1, gear_pos.1),
                Position(gear_pos.0 - 1, gear_pos.1 + 1),
                Position(gear_pos.0, gear_pos.1 - 1),
                Position(gear_pos.0, gear_pos.1 + 1),
                Position(gear_pos.0 + 1, gear_pos.1 - 1),
                Position(gear_pos.0 + 1, gear_pos.1),
                Position(gear_pos.0 + 1, gear_pos.1 + 1),
            ]
            .into_iter()
            // filter out positions that are out of bounds
            .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < num_rows as i32 && p.1 < num_cols as i32)
            .collect::<Vec<Position>>();

            // checking if number is in range of symbol
            for neighbor in neighbors.iter() {
                if neighbor.0 == start_position.0
                    && neighbor.1 >= start_position.1
                    && neighbor.1 <= num_end_pos.1
                {
                    let num: u32 = num.parse::<u32>().unwrap();

                    // storing gear position and neighbor number
                    gear_table
                        .entry(*gear_pos)
                        .and_modify(|v| v.push(num))
                        .or_insert(vec![num]);

                    break;
                }
            }
        }
    }

    // calculating gear ratios
    let mut gear_ratios: Vec<u32> = vec![];

    for v in gear_table.values() {
        // if there is two numbers, they are connected by a gear
        // and we can calculate the gear ratio
        if v.len() == 2 {
            gear_ratios.push(v.iter().product());
        }
    }
    gear_ratios.iter().sum()
}

// --- helpers ---
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Position(i32, i32);



fn main() {
    println!("=== Advent of Code 2023 - Day 3 ===\n");

    println!("--- Part 1 ---");

    println!("PART1: Test file");
    let result = part_1(EXAMPLE_INPUT_PART_1);
    let expected = 4361;
    println!("Test result: {} (expected: {})", result, expected);

    if result == expected {
        println!("Test successful!");
    } else {
        panic!("Test failed!");
    }

    println!("PART1: Input file");
    let result = part_1(PUZZLE_INPUT);
    println!("Result: {}", result);

    println!("--- Part 2 ---");

    println!("PART1: Test file");
    let result = part_2(EXAMPLE_INPUT_PART_2);
    let expected = 467835;
    println!("Test result: {} (expected: {})", result, expected);

    if result == expected {
        println!("Test successful!");
    } else {
        panic!("Test failed!");
    }

    println!("PART1: Input file");
    let result = part_2(PUZZLE_INPUT);
    println!("Result: {}", result);
}
