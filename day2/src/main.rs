use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: Move,
    mine: Move,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

enum Outcome_pt2 {
    Loss,
    Draw,
    Win,
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => panic!("String errada"),
        }
    }
}

impl Outcome {
    fn points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("at least one move beats us")
    }

    fn drawing_move(self) -> Self {
        self
    }

    fn points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
} 

impl Round {
    fn outcome(self) -> Outcome {
        self.mine.outcome(self.opponent)
    }

    fn my_score(self) -> usize {
        self.mine.points() + self.outcome().points()
    }
}

impl TryFrom<char> for Outcome_pt2 {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome_pt2::Loss),
            'Y' => Ok(Outcome_pt2::Draw),
            'Z' => Ok(Outcome_pt2::Win),
            _ => Err("Erro".to_string())
        }
    }
}

impl Outcome_pt2 {
    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome_pt2::Win => theirs.winning_move(),
            Outcome_pt2::Draw => theirs.drawing_move(),
            Outcome_pt2::Loss => theirs.losing_move(),
        }
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err("Failed to parse".to_string());
        };
        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome_pt2::try_from(outcome)?;
        let ours = outcome.matching_move(theirs);

        Ok(Self { opponent: theirs, mine: ours })
    }
}

fn main() -> Result<(), String> {
    let final_score: usize = include_str!("day2.txt")
        .lines().map(|line| line.parse::<Round>().unwrap())
        .map(|round| round.my_score())
        .sum();
    println!("{:?}", final_score);

    Ok(())
}
