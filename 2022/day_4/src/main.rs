use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

const INPUT_FILENAME: &str = "input.txt";

fn parse_error(line: &str) -> ! {
    panic!("Unparsable line {}", line)
}

fn into_range(range: &str) -> RangeInclusive<usize>  {
    let Some((start, end)) = range.split_once('-') else {
        parse_error(range);
    };

    start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
}

fn is_contained(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> bool {
    first.contains(second.start()) && first.contains(second.end())
}

fn is_overlap(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> bool {
    first.contains(second.start()) || first.contains(second.end())
}

fn main() {
    println!("Solution to Part A: {}", part_a());
    println!("Solution to Part B: {}", part_b());
}

fn part_a() -> usize {
    BufReader::new(File::open(INPUT_FILENAME).unwrap())
        .lines()
        .filter(|l| {
            let line = l.as_ref().unwrap();
            let Some((first, second)) = line.split_once(',') else {
                parse_error(line)
            };

            let (first, second) = (into_range(first), into_range(second));

            is_contained(&first, &second) || is_contained(&second, &first)
        })
        .count()
}


fn part_b() -> usize {
    BufReader::new(File::open(INPUT_FILENAME).unwrap())
        .lines()
        .filter(|l| {
            let line = l.as_ref().unwrap();
            let Some((first, second)) = line.split_once(',') else {
                parse_error(line)
            };

            let (first, second) = (into_range(first), into_range(second));

            is_overlap(&first, &second) || is_overlap(&second, &first)
        })
        .count()
}
