use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = get_input();

    let mut score = 0;
    for x in input {
        score += calculate_score(x.as_str());
    }

    println!("{}", score);
}

fn calculate_score(input: &str) -> i32 {
    let their_input = input.chars().nth(0).expect("Failed to get their char from string") as i32 - 'A' as i32;
    let my_input = input.chars().nth(2).expect("Failed to get my char from string") as i32 - 'X' as i32;

    let shape_points = my_input + 1;
    if my_input == their_input {
        return 3 + shape_points;
    }

    let rock = 0;
    let paper = 1;
    let scissor = 2;

    if my_input == rock && their_input != scissor {
        return shape_points;
    }

    if my_input == paper && their_input != rock {
        return shape_points;
    }

    if my_input == scissor && their_input != paper {
        return shape_points;
    }

    6 + shape_points
}

fn get_input() -> Vec<String> {
    let file = File::open("input.txt").expect("Failed to find input file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}