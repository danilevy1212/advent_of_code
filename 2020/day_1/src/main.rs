use std::io::{BufReader, Result};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() -> Result<()> {
    const TARGET: u32 = 2020;
    let input_set = parse_input()?;
    let answer = sum_of_three(&input_set, TARGET);

    match answer {
        Some((elt1, elt2, elt3)) => print!("{} x {} x {} = {}", elt1, elt2, elt3, elt1 * elt2 * elt3),
        None => eprintln!("No triplet found"),
    }

    Ok(())
}

/// Return the sum of three elements that results in `target`
fn sum_of_three(set: &HashSet<u32>, target: u32) -> Option<(u32, u32, u32)> {
    for elt1 in set.iter() {
        if *elt1 > target {
            continue;
        }

        let partial_sum = target - elt1;
        let other_two = sum_of_two(set, partial_sum);

        if let Some((elt2, elt3)) = other_two {
            return Some((*elt1, elt2, elt3))
        }
    }

    None
}

/// Return the sum of two elements that results in `target`
fn sum_of_two(set: &HashSet<u32>, target: u32) -> Option<(u32, u32)> {
    let answer = set.iter().find(|elt| -> bool {
        if **elt <= target {
            return set.contains(&(target - *elt))
        }

        false
    });

    answer.map(|elt| { (*elt, target - elt) })
}

/// Parse `input.txt` and return a `HashSet` of it's contents.
///
/// `input.txt` is assumed to be file containing
/// natural integer numbers separated by a breakline.
fn parse_input() -> Result<HashSet<u32>> {

    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);
    let mut result = HashSet::new();

    for line in reader.lines() {
        result.insert(line?.parse::<u32>().unwrap());
    }

    Ok(result)
}
