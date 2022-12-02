#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Shape {
    fn from(string: &str) -> Self {
        use Shape::*;
        match string {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!("invalid Shape: {string}"),
        }
    }
}

impl Shape {
    fn play(&self, other: &Self) -> Outcome {
        use Outcome::*;
        use Shape::*;

        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn round_score(&self, other: &Self) -> u32 {
        let outcome = self.play(other);
        let shape_score = self.score();
        let outcome_score = outcome.score();

        shape_score + outcome_score
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let rounds: Vec<(Shape, Shape)> = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opponent, you)| (Shape::from(opponent), Shape::from(you)))
        .collect();

    let part1: u32 = rounds
        .iter()
        .map(|(opponent, you)| you.round_score(opponent))
        .sum();

    println!("part1 = {part1}");
}
