use std::str::FromStr;

struct Assignment {
    start: u32,
    end: u32,
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").unwrap();

        return Ok(Assignment {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        });
    }
}

pub fn process_part1(input: &str) -> String {
    let result: i32 = input.lines().map(|pair| {
        let (elf1, elf2) = pair.split_once(",").unwrap();
        let elf1: Assignment = elf1.parse().unwrap();
        let elf2: Assignment = elf2.parse().unwrap();

        if elf1.start >= elf2.start && elf1.end <= elf2.end {
            return 1;
        } 
        if elf2.start >= elf1.start && elf2.end <= elf1.end {
            return 1;
        }
        return 0;
    }).sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: i32 = input.lines().map(|pair| {
        let (elf1, elf2) = pair.split_once(",").unwrap();
        let elf1: Assignment = elf1.parse().unwrap();
        let elf2: Assignment = elf2.parse().unwrap();
        
        if elf1.start >= elf2.start && elf1.start <= elf2.end {
            return 1;
        } 
        if elf2.start >= elf1.start && elf2.start <= elf1.end {
            return 1;
        } 
        return 0;
        
    }).sum();

    result.to_string()
}
