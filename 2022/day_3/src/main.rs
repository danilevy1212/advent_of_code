use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    println!("Solution to Part A: {}", part_a());
    println!("Solution to Part B: {}", part_b());
}

fn part_a() -> u32 {
    BufReader::new(File::open(INPUT_FILENAME).unwrap())
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let (first, second) = line.split_at(line.len() / 2);

            for l in first.chars() {
                if second.find(l).is_some() {
                    // NOTE: I'm offsetting ASCII values https://www.asciitable.com/
                    return Some(match l.is_uppercase() {
                        true => l as u32 - 38,
                        false => l as u32 - 96,
                    });
                }
            }

            None
        })
        .sum()
}

fn part_b() -> u32 {
    BufReader::new(File::open(INPUT_FILENAME).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|group| {
            let first = group[0].clone();
            let second = group[1].clone();
            let third = group[2].clone();

            let mut intersection = Vec::new();
            for l in first.chars() {
                if second.find(l).is_some() {
                    intersection.push(l)
                }
            }

            for l in intersection {
                if third.find(l).is_some() {
                    // NOTE: I'm offsetting ASCII values https://www.asciitable.com/
                    return Some(match l.is_uppercase() {
                        true => l as u32 - 38,
                        false => l as u32 - 96,
                    });
                }
            }

            None
        })
        .sum()
}
