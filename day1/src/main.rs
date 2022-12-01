use std::fs;

fn main() {
    // PART 1
    let mut elf_calories = get_elf_calories();

    let elf_with_most_calories = find_elf_with_most_calories(&elf_calories);
    println!("Elf with most calories is: {} with: {} calories.", elf_with_most_calories, elf_calories[elf_with_most_calories]);

    // PART 2
    let mut top3_calories_total = 0;
    for _ in 0..3 {
        let index = find_elf_with_most_calories(&elf_calories);
        top3_calories_total += elf_calories[index];
        elf_calories.remove(index);
    }
    println!("The top 3 elves got a total of {} calories!", top3_calories_total);
}

fn get_elf_calories() -> Vec<i32> {
    let input_bytes = include_bytes!("../input.txt");
    let input = String::from_utf8_lossy(input_bytes);

    let lines : Vec<&str> = input.split('\n').collect();
    let mut calories_per_elf: Vec<i32> = Vec::new();

    let mut current_calories = 0;
    for line in lines {
        let sanitized_line = line.replace('\r', "");
        
        if sanitized_line.is_empty() {
            calories_per_elf.push(current_calories);
            current_calories = 0;
            continue;
        }
        
        let line_value: i32 = sanitized_line.parse::<i32>().expect("Failed to parse line");
        current_calories += line_value;
    }

    calories_per_elf
}

fn find_elf_with_most_calories(elf_calories: &Vec<i32>) -> usize {
    let index: Option<usize> = elf_calories
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)|index);

    index.expect("Entered empty vector.")
}