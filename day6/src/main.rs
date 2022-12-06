use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = get_input();

    let index = find_index_of_first_four_unique(&input, 4);
    println!("Part 1: {}", index + 1);

    // -------- PART 2 -----------
    let index2 = find_index_of_first_four_unique(&input, 14);
    println!("Part 2: {}", index2 + 1);
}

fn find_index_of_first_four_unique(input: &Vec<String>, num_unique: usize) -> i32 {
    let string = input[0].clone();

    let mut chars: Vec<char> = Vec::new();
    for index in 0..string.len() {
        let char = string.chars().nth(index).expect("Ran out of chars.");

        if chars.len() != num_unique {
            chars.push(char);

            if chars.len() == num_unique {
                if !has_duplicates(&chars, num_unique) {
                    return 0;
                }
            }

            continue;
        }

        chars.rotate_left(1);
        chars.remove(chars.len() - 1);
        chars.push(char);

        if !has_duplicates(&chars, num_unique) {
            return index as i32;
        }
    }

    -1
}

fn has_duplicates(chars: &Vec<char>, num_unique: usize) -> bool {
    for index in 0..num_unique {
        let char = chars[index].clone();

        for index2 in 0..num_unique {
            if index == index2 {
                continue;
            }
            let char2 = chars[index2].clone();

            if char == char2 {
                return true;
            }
        }
    }

    return false;
}

fn get_input() -> Vec<String> {
    let file = File::open("input.txt").expect("Failed to find input file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
