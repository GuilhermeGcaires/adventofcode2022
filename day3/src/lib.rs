use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .map(|line| {
            let (comp_1, comp_2) = line.split_at(line.len() / 2);
            let common = comp_1.chars().find(|c| comp_2.contains(*c)).unwrap();
            letter_scores.get(&common).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .into_iter()
        .map(|chunk| { 
            let first = chunk[0];
            let second = chunk[1];
            let third = chunk[2];
            let common = first.chars().find(|c| second.contains(*c) && third.contains(*c)).unwrap();
            letter_scores.get(&common).unwrap()
        }).sum::<usize>();

    result.to_string()
}

