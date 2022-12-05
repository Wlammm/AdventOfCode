use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = get_input();

    {
        let mut stacks = get_stacks();

        // --------------- PART 1 --------------------------
        for line in &input {
            let words: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
            
            let amount = words[1].parse::<i32>().expect("Failed to parse amount.");
            let start_stack = words[3].parse::<usize>().expect("Failed to parse start stack") - 1;
            let end_stack = words[5].parse::<usize>().expect("Failed to parse end stack") - 1;
            stack_move(&mut stacks, amount, start_stack, end_stack);
        }

        print_result(&mut stacks);
    }

    // ----------------- PART 2 ---------------------
    let mut stacks = get_stacks();

    for line in &input {
        let words: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        
        let amount = words[1].parse::<i32>().expect("Failed to parse amount.");
        let start_stack = words[3].parse::<usize>().expect("Failed to parse start stack") - 1;
        let end_stack = words[5].parse::<usize>().expect("Failed to parse end stack") - 1;
        stack_move2(&mut stacks, amount as usize, start_stack, end_stack);
    }

    print_result(&mut stacks);
}

fn get_stacks() -> Vec<Vec<char>> {
    let mut stacks:Vec<Vec<char>> = Vec::new();
    stacks.push(vec!('S', 'C', 'V', 'N'));
    stacks.push(vec!('Z', 'M', 'J', 'H', 'N', 'S'));
    stacks.push(vec!('M', 'C', 'T', 'G', 'J', 'N', 'D'));
    stacks.push(vec!('T', 'D', 'F', 'J', 'W', 'R', 'M'));
    stacks.push(vec!('P', 'F', 'H'));
    stacks.push(vec!('C', 'T', 'Z', 'H', 'J'));
    stacks.push(vec!('D', 'P', 'R', 'Q', 'F', 'S', 'L', 'Z'));
    stacks.push(vec!('C', 'S', 'L', 'H', 'D', 'F', 'P', 'W'));
    stacks.push(vec!('D', 'S', 'M', 'P', 'F', 'N', 'G', 'Z'));
    stacks
}

fn print_result(stacks: &mut Vec<Vec<char>>) {
    let mut result = String::from("");
    for x in stacks {
        let last_char = *x.last().unwrap();
        result.push(last_char);
    }

    println!("{}", result);
}

fn stack_move(stacks: &mut Vec<Vec<char>>, amount: i32, from: usize, to: usize) {
    for _ in 0..amount {
        let char = *stacks[from].last().unwrap();
        stacks[to].push(char);
        let last_index = stacks[from].len() - 1;
        stacks[from].remove(last_index);
    }
}

fn stack_move2(stacks: &mut Vec<Vec<char>>, amount: usize, from: usize, to: usize) {
    let from_length = stacks[from].len();

    for x in from_length-amount..from_length {
        let char = stacks[from][x].clone();
        stacks[to].push(char);
    }

    for _ in 0..amount {
        let last_index = stacks[from].len() - 1;
        stacks[from].remove(last_index);
    }
}

fn get_input() -> Vec<String> {
    let file = File::open("input.txt").expect("Failed to find input file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
