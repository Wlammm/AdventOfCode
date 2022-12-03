use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = get_input();

    let mut score = 0;
    for line in &input {
        score += get_score_of_rucksack(line.as_str());
    }

    println!("{}", score);

    // ------------ PART 2 --------------
    score = 0;
    for i in 0..input.len() / 3 {
        let index = i * 3;
        let char = find_item_group(&input[index], &input[index + 1], &input[index + 2]);
        score += get_score_of_char(char);
    }
    println!("{}", score);
}

fn find_item_group(input: &str, input2: &str, input3: &str) -> char {
    for char in input.chars() {
        if input2.contains(char) && input3.contains(char) {
            return char;
        }
    }

    ' '
}

fn get_score_of_rucksack(input: &str) -> i32 {
    let half_size = input.len() / 2;

    let (first_compartment, second_compartment) = input.split_at(half_size);

    let mut chars: Vec<char> = Vec::new();
    for char in first_compartment.chars() {
        if second_compartment.contains(char) {
            if !chars.contains(&char) {
                chars.push(char);
            }
        }
    }

    let mut score = 0;
    for x in chars {
        score += get_score_of_char(x);
    }

    score
}

fn get_score_of_char(input: char) -> i32 {
    if input.is_lowercase() {
        return input as i32 - 'a' as i32 + 1;
    }

    return input as i32 - 'A' as i32 + 27;
}

fn get_input() -> Vec<String> {
    let file = File::open("input.txt").expect("Failed to find input file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
