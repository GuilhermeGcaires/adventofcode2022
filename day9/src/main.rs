use std::{fs, collections::HashSet, vec};

struct Rope {
    segments: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>
}

impl Rope {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn new(len: usize) -> Self {
        Self { segments: vec![(0, 0); len], visited: HashSet::new() }
    }

    fn make_move(&mut self, dir: &Direction) {
        let delta = Self::DIR[*dir as usize];
        self.segments[0].0 += delta.0;
        self.segments[0].1 += delta.1;

        for i in 1..self.segments.len() {
            let rowdiff = self.segments[i - 1].0 - self.segments[i].0;
            let coldiff = self.segments[i - 1].1 - self.segments[i].1;

            if rowdiff.abs() > 1 || coldiff.abs() > 1 {
                self.segments[i].0 += rowdiff.signum();
                self.segments[i].1 += coldiff.signum();
            }
        }
        self.visited.insert(self.segments[self.segments.len() - 1]);
    }

}


#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("invalid direction '{s}'"),
        }
    }
}

fn main() {
    let mut file = fs::read_to_string("./src/input.txt").expect("Could not read file");
    let mut steps: Vec<(Direction, i32)> = Vec::new();

    for line in file.lines() {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dir = Direction::parse(dir);
        let dist = dist.parse().unwrap();
        steps.push((dir, dist));
        }

    let mut snake = Rope::new(10);

    for (dir, amount) in &steps {
        for _ in 0..*amount {
            snake.make_move(dir);
        }
    }


    println!("{steps:?}");
    println!("{:?}", snake.visited.len())
}
