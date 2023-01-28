use std::ops::Index;

#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<u32>>,
}

impl Forest {
    fn new(input: Vec<Vec<u32>>) -> Forest {
        Forest { grid: input }
    }

    fn is_visible(&self, x: usize, y: usize) -> [Vec<u32>; 4]{
        let row = &self.grid[y].clone();
        let column = self.grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

        let (left, right) = row.split_at(x);
        let (up, down) = column.split_at(y);

        let up: Vec<u32> = up.iter().copied().rev().collect();
        let left: Vec<u32>  = left.iter().copied().rev().collect();
        let right: Vec<u32>  = right[1..].to_vec();
        let down: Vec<u32>  = down[1..].to_vec();

        [up, down, left, right]
    }
}

fn parser(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

pub fn process_part1(input: &str) -> String {
    let trees = parser(&input);
    let mut forest = Forest::new(trees);

    for row in 1..forest.grid.len() - 1 {
        for col in 1..forest.grid.index(0).len() - 1 {
            forest.is_visible(row, col);
            
        }
    }

    "result".to_string()
}

pub fn process_part2(input: &str) -> String {
    todo!();
}
