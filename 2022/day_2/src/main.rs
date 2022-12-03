use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum State {
    Win,
    Draw,
    Lose,
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Unknown hand {}", s),
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            // WINNING
            (Self::Rock, Self::Scissors) => Greater,
            (Self::Scissors, Self::Paper) => Greater,
            (Self::Paper, Self::Rock) => Greater,
            // LOOSING
            (Self::Scissors, Self::Rock) => Less,
            (Self::Paper, Self::Scissors) => Less,
            (Self::Rock, Self::Paper) => Less,
            // TIE
            _ => Equal,
        })
    }
}

impl Hand {
    fn to_score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn with_state(&self, s: &State) -> Self {
        match (self, s) {
            // WINNING
            (Self::Scissors, State::Win) => Self::Rock,
            (Self::Rock, State::Win) => Self::Paper,
            (Self::Paper, State::Win) => Self::Scissors,
            // LOOSING
            (Self::Scissors, State::Lose) =>Self::Paper,
            (Self::Paper, State::Lose) => Self::Rock,
            (Self::Rock, State::Lose) => Self::Scissors,
            // TIE
            (h, State::Draw) => h.to_owned(),
        }
    }
}

fn to_score(o: &Ordering) -> u32 {
    match o {
        Greater => 6,
        Equal => 3,
        Less => 0,
    }
}

impl FromStr for State {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unknown state {}", s)
        })
    }
}

impl State {
    fn to_score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

fn part_a() -> u32 {
    let input_file = File::open(INPUT_FILENAME).unwrap();
    let reader = BufReader::new(input_file);
    let mut total: u32 = 0;

    for line in reader.lines() {
        let score = line.unwrap()
            .split_once(' ')
            .map(|(other, me)| {
               (other.parse::<Hand>().unwrap(), me.parse::<Hand>().unwrap())
            })
            .map(|(other, me)| {
               to_score(&me.partial_cmp(&other).unwrap()) + me.to_score()
            })
            .unwrap();

        total += score;
    }

    total
}

fn part_b() -> u32 {
    let input_file = File::open(INPUT_FILENAME).unwrap();
    let reader = BufReader::new(input_file);
    let mut total: u32 = 0;

    for line in reader.lines() {
        let score = line.unwrap()
            .split_once(' ')
            .map(|(other, state)| {
                (other.parse::<Hand>().unwrap(), state.parse::<State>().unwrap())
            })
            .map(|(other, state)| {
                other.with_state(&state).to_score() + state.to_score()
            })
            .unwrap();

        total += score;
    }

    total
}

fn main() {
    println!("Part A solution: {}", part_a());
    println!("Part B solution: {}", part_b());
}
