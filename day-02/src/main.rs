use Shape::*;
use Outcome::*;

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Shape {
    fn from(string: &str) -> Self {
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

    fn what_to_play(&self, desired_outcome: Outcome) -> Self {
        match (self, desired_outcome) {
            (Rock, Win) => Paper,
            (Rock, Lose) => Scissors,
            (Rock, Draw) => Rock,
            (Paper, Win) => Scissors,
            (Paper, Lose) => Rock,
            (Paper, Draw) => Paper,
            (Scissors, Win) => Rock,
            (Scissors, Lose) => Paper,
            (Scissors, Draw) => Scissors,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<&str> for Outcome {
    fn from(string: &str) -> Self {
        match string {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!("invalid Outcome: {string}"),
        }
    }
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
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let part1: u32 = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opponent, you)| (Shape::from(opponent), Shape::from(you)))
        .map(|(opponent, you)| you.round_score(&opponent))
        .sum();

    println!("part1 = {part1}");
}

fn part2(input: &str) {
    let part2: u32 = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opponent, desired_outcome)| {
            let opponent = Shape::from(opponent);
            let desired_outcome = Outcome::from(desired_outcome);
            let should_play = opponent.what_to_play(desired_outcome);
            (opponent, should_play)
        })
        .map(|(opponent, you)| you.round_score(&opponent))
        .sum();

    println!("part2 = {part2}");
}
