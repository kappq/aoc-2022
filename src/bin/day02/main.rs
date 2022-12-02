use std::{fs, str::FromStr};

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => panic!("invalid input"),
        }
    }
}

fn first_strategy(line: &str) -> u32 {
    let mut columns = line.split_whitespace();
    let (opponent, you) = (
        Shape::from_str(columns.next().unwrap()).unwrap(),
        Shape::from_str(columns.next().unwrap()).unwrap(),
    );

    let outcome = match (you, opponent) {
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => Outcome::Loss,
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => Outcome::Win,
    };

    you as u32 + outcome as u32
}

fn second_strategy(line: &str) -> u32 {
    let mut columns = line.split_whitespace();
    let (opponent, outcome) = (
        Shape::from_str(columns.next().unwrap()).unwrap(),
        Outcome::from_str(columns.next().unwrap()).unwrap(),
    );

    let you = match (opponent, outcome) {
        (Shape::Paper, Outcome::Loss)
        | (Shape::Rock, Outcome::Draw)
        | (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Loss)
        | (Shape::Paper, Outcome::Draw)
        | (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Loss)
        | (Shape::Scissors, Outcome::Draw)
        | (Shape::Paper, Outcome::Win) => Shape::Scissors,
    };

    you as u32 + outcome as u32
}

fn main() {
    let contents = fs::read_to_string("src/bin/day02/input.txt").expect("file not found");

    let first_star = contents.lines().map(first_strategy).sum::<u32>();
    let second_star = contents.lines().map(second_strategy).sum::<u32>();

    println!("first star: {}", first_star);
    println!("second star: {}", second_star);
}
