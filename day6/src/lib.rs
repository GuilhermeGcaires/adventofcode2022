use std::collections::HashMap;


pub fn process_part1(input: &str) -> String {
    let a = input.chars().collect::<Vec<char>>();
    
    for (i, win) in a.windows(4).enumerate() {
        let mut per_window = HashMap::new();

        for char in win {
            let count = per_window.entry(char).or_insert(0);
            *count += 1;
        }
        if per_window.len() == 4 {
            return (i+4).to_string();
        }
    }
    "falhou".to_string()
}

pub fn process_part2(input: &str) -> String {
    let a = input.chars().collect::<Vec<char>>();
    
    for (i, win) in a.windows(14).enumerate() {
        let mut per_window = HashMap::new();

        for char in win {
            let count = per_window.entry(char).or_insert(0);
            *count += 1;
        }
        if per_window.len() == 14 {
            return (i+14).to_string();
        }
    }
    "falhou".to_string()
}
