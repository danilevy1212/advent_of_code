use std::{fs::File, collections::HashSet, iter::FromIterator};
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut res = reader
        .lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|line| to_id(&line))
        .collect::<Vec<_>>();

    res.sort();

    let max = res.last().unwrap();

    // PART 1
    println!("max: {}", max);

    // PART 2
    let min = res.first().unwrap();

    let res = HashSet::<u16>::from_iter(res.iter().cloned());

    let mut all_inrange = HashSet::new();

    for i in *min..(max + 1) {
        all_inrange.insert(i);
    }

    let my_seat = all_inrange.iter().find(|x| {
        !res.contains(x)
    });

    println!("My seat: {}", my_seat.unwrap());

    Ok(())
}

fn to_id(raw_string: &String) -> Option<u16> {
    if raw_string.len() != 10 {
        return None;
    }

    let id_str = raw_string
        .chars()
        .filter_map(|c| match c {
            'R' => Some('1'),
            'B' => Some('1'),
            'L' => Some('0'),
            'F' => Some('0'),
            _ => None,
        })
        .collect::<String>();

    if id_str.len() != 10 {
        return None;
    }

    Some(u16::from_str_radix(&id_str, 2).unwrap(),)
}
