use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = get_input();

    let mut score = 0;
    for x in &input {
        score += calculate_score(x.as_str());
    }

    println!("First part: {}", score);

    // part 2
    let mut score2 = 0;
    for x in &input {
        score2 += calculate_score_second_part(x.as_str());
    }

    println!("Second part: {}", score2);
}

fn get_winning_char(input: char) -> char {
    match input {
        'A' => return 'Y',
        'B' => return 'Z',
        'C' => return 'X',
        _ => return ' ',
    }
}

fn get_loosing_char(input: char) -> char {
    match input {
        'A' => return 'Z',
        'B' => return 'X',
        'C' => return 'Y',
        _ => return ' ',
    }
}

fn calculate_score_second_part(input: &str) -> i32 {
    let my_input = input.chars().nth(2).expect("Failed to get my char from string");
    let their_char = input.chars().nth(0).expect("Failed to get their char from string");

    // lose
    if my_input == 'X' {
        let shape_points = get_loosing_char(their_char) as i32 - 'X' as i32;
        return shape_points + 1;
    }

    // tie
    if my_input == 'Y' {
        let shape_points = their_char as i32 - 'A' as i32;
        return 3 + shape_points + 1;
    }

    let shape_points = get_winning_char(their_char) as i32 - 'X' as i32;
    shape_points + 1 + 6
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