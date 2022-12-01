use std::fs;

fn main() {
    let elf_calories = get_elf_calories();

    for a in elf_calories {
        println!("{}", a);
    }
}

fn get_elf_calories() -> Vec<i32> {
    let input_bytes = include_bytes!("../input.txt");
    let input = String::from_utf8_lossy(input_bytes);

    let lines : Vec<&str> = input.split('\n').collect();

    let mut calories_per_elf: Vec<i32> = Vec::new();    

    let mut current_calories = 0;
    for line in lines {
        if line.is_empty() {
            calories_per_elf.push(current_calories);
            current_calories = 0;
        }

        let line_value: i32 = line.parse::<i32>().expect("Failed to parse line");
    }

    calories_per_elf
}