use std::{str::FromStr, collections::VecDeque}; 

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parser(input: &str) -> Vec<VecDeque<char>> {
    let mut cargo: Vec<VecDeque<char>> = Vec::new();
    let mut instructions: Vec<Move> = Vec::new();
    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<_>>();
        if line.contains('[') {
            for item in line.chars().enumerate().filter(|(i, c)| *c != ' ' && (i + 3) % 4 == 0) {
                let stack = (item.0 - 1) / 4; 
                while cargo.len() < stack + 1 {
                    cargo.push(VecDeque::new());
                }
                cargo[stack].push_front(item.1);
            }
        } else if words[0] == "move" {
            let amount: usize = words[1].parse().unwrap();
            let from: usize = words[3].parse::<usize>().unwrap() - 1;
            let to: usize = words[5].parse::<usize>().unwrap() - 1;
            instructions.push(Move { amount, from, to }) 
        }
    }

    let result = compute_part2(cargo, instructions);
    result
}

fn compute_part1(mut cargo: Vec<VecDeque<char>>, instruction: Vec<Move>) -> Vec<VecDeque<char>> {
   for i in instruction {
        for _ in 0..i.amount {
            let container = cargo[i.from].pop_back().unwrap();
            cargo[i.to].push_back(container);
        }
    } 
    cargo
}

pub fn process_part1(input: &str) -> String {
    let result = parser(input);
    let mut ans: String = "".to_string();
    
    for mut i in result {
        ans.push(i.pop_back().unwrap());
    }
    ans.to_string()
}

fn compute_part2(mut cargo: Vec<VecDeque<char>>, instruction: Vec<Move>) -> Vec<VecDeque<char>> {
    for i in instruction {
        let split_at = cargo[i.from].len() - i.amount;
        let mut removed = cargo[i.from].split_off(split_at);
        cargo[i.to].append(&mut removed);
    } 
    cargo
}

pub fn process_part2(input: &str) -> String {
    let result = parser(input);
    let mut ans: String = "".to_string();
    
    for mut i in result {
        ans.push(i.pop_back().unwrap());
    }
    ans.to_string()
}
