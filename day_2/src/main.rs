use std::{fs::read_to_string, collections::HashMap};

const PUZZLE_INPUT: &str = "input.txt";
const EXAMPLE_INPUT_PART_1: &str = "example1.txt";
const EXAMPLE_INPUT_PART_2: &str = "example2.txt";

// --- execution parts ---
fn part_1(file_path: &str, threshold_map: ThresholdMap) -> i32 {
    // parse file
    let parsed_games = parse_game_file(file_path);

    // iterate over games and filter operation result
    parsed_games
    .iter()
    .filter_map(|game| {
        // get game ID, if the game is valid it will be added
        let game_id = game.id;

        let mut iterator = 0;
        let mut over_limit = false;

        // loop over draws and check if any of the values is over the threshold
        // if it is, the game is invalid
        while (iterator < game.draws.len()) && !over_limit {
            let draw = &game.draws[iterator];

            let red = draw.red;
            let green = draw.green;
            let blue = draw.blue;

            let red_threshold = threshold_map.get("red").unwrap();
            let green_threshold = threshold_map.get("green").unwrap();
            let blue_threshold = threshold_map.get("blue").unwrap();

            if red > *red_threshold || green > *green_threshold || blue > *blue_threshold {
                over_limit = true;
            }

            iterator += 1;
        }

        // if the game is valid, return the game ID for the sum
        if !over_limit {
            Some(game_id)
        } else {
            None
        }
    })
    .sum()
}

fn part_2(file_path: &str) -> i32 {
        // parse file
        let parsed_games = parse_game_file(file_path);

        // iterate over games and filter operation result
        parsed_games
        .iter()
        .map(|game| {
            // setup a zeroed map of color to store max values for each color
            let mut max_values = create_threshold_map(0, 0, 0);

            // iterate over draws and check if any of the values is over the threshold
            // if it is, replace the value in the map with the current value
            for draw in &game.draws {
                let red = draw.red;
                let green = draw.green;
                let blue = draw.blue;

                // BOOOOOOOOOOOOOO clone is for squares
                // but did not wanted to deal with lifetimes so be it
                let cloned = max_values.clone();

                // getting current max values
                let red_threshold = cloned.get("red").unwrap();
                let green_threshold = cloned.get("green").unwrap();
                let blue_threshold = cloned.get("blue").unwrap();

                // checking if current value is greater than the threshold
                if red > *red_threshold {
                    max_values.insert("red".to_string(), red);
                }
                if green > *green_threshold {
                    max_values.insert("green".to_string(), green);
                }
                if blue > *blue_threshold {
                    max_values.insert("blue".to_string(), blue);
                }
            }

            // get the max values from the map
            let red_threshold = max_values.get("red").unwrap();
            let green_threshold = max_values.get("green").unwrap();
            let blue_threshold = max_values.get("blue").unwrap();

            // multiply the max values
            red_threshold * green_threshold * blue_threshold
        })
        // sum it
        .sum()
}

// --- Helpers ---
type ThresholdMap = HashMap<String, i32>;

fn create_color_vec<'a>() -> Vec<&'a str> {
    ["red", "green", "blue"].to_vec()
}

#[derive(Debug, Default)]
struct Draw {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

#[derive(Debug, Default)]
struct Game {
    pub id: i32,
    pub draws: Vec<Draw>,
}

fn parse_game_file(filepath: &str) -> Vec<Game> {
    // create a vector of games
    let mut games: Vec<Game> = Vec::new();

    // get colors
    let colors = create_color_vec();

    // read file
    let lines = read_to_string(filepath).expect("Could not read file");

    // iterate over lines
    for line in lines.lines() {
        // create the current line game
        let mut current_game = Game::default();

        // split game info and draws
        let mut game_info_draws = line.split(':');
        let game_info = game_info_draws.next().unwrap();
        let draws = game_info_draws.next().unwrap();

        // split game info
        let game_id = game_info
        .replace("Game ", "")
        .parse::<i32>()
        .unwrap();

        // set game id
        current_game.id = game_id;

        // split draws
        let draws = draws.split(';');

        // iterate over draws
        for draw in draws {
            // create the current draw
            let mut current_draw = Draw::default();

            // split draw
            let draw = draw.split(',');

            // iterate over draw
            for possibilities in draw {
                // checking color of draw
                for color in &colors {
                    // if color is in possibilities
                    if possibilities.contains(color) {
                        // get value
                        let color_pattern = format!(" {}", color);
                        let possible_integer = possibilities.replace(&color_pattern, "");

                        let value = possible_integer
                        .replace(' ', "")
                        .parse::<i32>()
                        .unwrap();

                        match *color {
                            "red" => current_draw.red = value,
                            "green" => current_draw.green = value,
                            "blue" => current_draw.blue = value,
                            _ => (),
                        }
                    }
                }
            }

            // add draw to game
            current_game.draws.push(current_draw);
        }

        // add game to games
        games.push(current_game);
    }

    // return the vector of games
    games
}

fn create_threshold_map(red: i32, green: i32, blue: i32) -> ThresholdMap {
    let mut threshold_map = HashMap::new();

    threshold_map.insert("red".to_string(), red);
    threshold_map.insert("green".to_string(), green);
    threshold_map.insert("blue".to_string(), blue);

    threshold_map
}

fn main() {
    println!("=== Advent of Code 2023 - Day 2 ===\n");

    println!("--- Part 1 ---");

    println!("PART1: Test file");
    let result = part_1(EXAMPLE_INPUT_PART_1, create_threshold_map(12, 13, 14));
    let expected = 8;
    println!("Test result: {} (expected: {})", result, expected);

    if result == expected {
        println!("Test successful!");
    } else {
        panic!("Test failed!");
    }

    println!("PART1: Input file");
    let result = part_1(PUZZLE_INPUT, create_threshold_map(12, 13, 14));
    println!("Result: {}", result);

    println!("--- Part 2 ---");

    println!("PART2: Test file");
    let result = part_2(EXAMPLE_INPUT_PART_2);
    let expected = 2286;
    println!("Test result: {} (expected: {})", result, expected);

    if result == expected {
        println!("Test successful!");
    } else {
        panic!("Test failed!");
    }

    println!("PART2: Input file");
    let result = part_2(PUZZLE_INPUT);
    println!("Result: {}", result);
}
