use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct Instructions {
    amount: usize,
    source: usize,
    target: usize,
}

impl FromStr for Instructions {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let curation = s
            .replace("move ", " ")
            .replace(" from ", " ")
            .replace(" to ", " ");
        let parts = curation.trim().split(' ').collect::<Vec<&str>>();
        let [amount, source, target] = parts.as_slice() else {
                panic!("Imposible pattern");
            };

        Ok(Instructions {
            amount: amount.parse().unwrap(),
            source: source.parse::<usize>().unwrap() - 1,
            target: target.parse::<usize>().unwrap() - 1,
        })
    }
}

fn main() {
    println!("Solution to Part A: {}", part_a());
    println!("Solution to Part B: {}", part_b());
}

fn part_a() -> String {
    let mut buffer_reader = BufReader::new(File::open(INPUT_FILENAME).unwrap())
        .lines()
        .peekable();

    // Per crane, 3 chars and n - 1 spaces
    let n_crates = (buffer_reader.peek().unwrap().as_ref().unwrap().len() + 1) / 4;

    // Initialize crates
    let mut stacks: Vec<LinkedList<char>> = Vec::new();
    for _ in 0..n_crates {
        stacks.push(LinkedList::new())
    }

    // Parse crates
    while buffer_reader
        .peek()
        .map(|x| !x.as_ref().unwrap().is_empty())
        .unwrap()
    {
        let line = buffer_reader.next().unwrap().unwrap();

        // Index into the char we want
        for (i, stack) in stacks.iter_mut().enumerate().take(n_crates) {
            let cargo = line.chars().nth(1 + i * 4).unwrap();

            // Guard against cargo line
            if cargo.is_ascii_alphabetic() {
                stack.push_back(cargo);
            }
        }
    }

    // Skip empty line
    buffer_reader.next();

    // Parse moves
    for line in buffer_reader {
        let instructions = line.unwrap().parse::<Instructions>().unwrap();

        for _ in 0..instructions.amount {
            let moved_crate = stacks[instructions.source].pop_front().unwrap();
            stacks[instructions.target].push_front(moved_crate);
        }
    }

    stacks.into_iter().map(|stack| {
        let last_crane = *stack.front().unwrap();

        last_crane
    }).collect::<String>()
}

fn part_b() -> String {
    let mut buffer_reader = BufReader::new(File::open(INPUT_FILENAME).unwrap())
        .lines()
        .peekable();

    // Per crane, 3 chars and n - 1 spaces
    let n_crates = (buffer_reader.peek().unwrap().as_ref().unwrap().len() + 1) / 4;

    // Initialize crates
    let mut stacks: Vec<LinkedList<char>> = Vec::new();
    for _ in 0..n_crates {
        stacks.push(LinkedList::new())
    }

    // Parse crates
    while buffer_reader
        .peek()
        .map(|x| !x.as_ref().unwrap().is_empty())
        .unwrap()
    {
        let line = buffer_reader.next().unwrap().unwrap();

        // Index into the char we want
        for (i, stack) in stacks.iter_mut().enumerate().take(n_crates) {
            let cargo = line.chars().nth(1 + i * 4).unwrap();

            // Guard against cargo line
            if cargo.is_ascii_alphabetic() {
                stack.push_back(cargo);
            }
        }
    }

    // Skip empty line
    buffer_reader.next();

    // Parse moves
    for line in buffer_reader {
        let instructions = line.unwrap().parse::<Instructions>().unwrap();
        let mut crates = LinkedList::new();

        for _ in 0..instructions.amount {
            let moved_crate = stacks[instructions.source].pop_front().unwrap();
            crates.push_front(moved_crate);
        }

        for c in crates.iter() {
            stacks[instructions.target].push_front(*c)
        }
    }

    stacks.into_iter().map(|stack| {
        let last_crane = *stack.front().unwrap();

        last_crane
    }).collect::<String>()
}
