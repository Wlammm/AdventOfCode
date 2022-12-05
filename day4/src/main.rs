use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = get_input();

    let mut overlaps = 0;
    for line in &input {
        if contains_other(line) {
            overlaps += 1;
        }
    }

    println!("Part 1: {}",  overlaps);

    // ----------- PART 2 -----------
    overlaps = 0;
    for line in &input {
        if contains_other_second_part(line) {
            overlaps += 1;
        }
    }

    println!("Part 2: {}",  overlaps);
}

fn contains_other_second_part(input: &str) -> bool {
    let (first, second) = input.split_once(',').expect("Failed to split data.");
    
    let (first_start, first_end) = first.split_once('-').expect("Failed to split range");
    let (second_start, second_end) = second.split_once('-').expect("Failed to split range.");

    let first_start_num = first_start.parse::<i32>().expect("Failed to parse string to int");
    let first_end_num = first_end.parse::<i32>().expect("Failed to parse string to int");
    
    let second_start_num = second_start.parse::<i32>().expect("Failed to parse string to int");
    let second_end_num = second_end.parse::<i32>().expect("Failed to parse string to int");

    if first_start_num <= second_start_num && first_end_num >= second_start_num || first_start_num <= second_end_num && first_end_num >= second_end_num {
        return true;
    }
     
    if second_start_num <= first_start_num && second_end_num >= first_start_num || second_start_num <= first_end_num && second_end_num >= first_end_num {
        return true;
    }

    false
}

fn contains_other(input: &str) -> bool {
    let (first, second) = input.split_once(',').expect("Failed to split data.");
    
    let (first_start, first_end) = first.split_once('-').expect("Failed to split range");
    let (second_start, second_end) = second.split_once('-').expect("Failed to split range.");

    let first_start_num = first_start.parse::<i32>().expect("Failed to parse string to int");
    let first_end_num = first_end.parse::<i32>().expect("Failed to parse string to int");
    
    let second_start_num = second_start.parse::<i32>().expect("Failed to parse string to int");
    let second_end_num = second_end.parse::<i32>().expect("Failed to parse string to int");

    if first_start_num <= second_start_num && first_end_num >= second_end_num || second_start_num <= first_start_num && second_end_num >= first_end_num {
        return true;
    }

    false
}

fn get_input() -> Vec<String> {
    let file = File::open("input.txt").expect("Failed to find input file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
