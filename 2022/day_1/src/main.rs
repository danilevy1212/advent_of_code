use std::{
    collections::BinaryHeap,
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const INPUT_FILENAME: &str = "input.txt";
const MAX_LENGTH: usize = 3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(INPUT_FILENAME).exists() {
        panic!(
            "File: {}/{} missing",
            current_dir()?.display(),
            INPUT_FILENAME
        )
    }

    println!("Part A solution: {}", part_a()?);
    println!("Part B solution: {}", part_b()?);

    Ok(())
}

fn part_a() -> Result<u32, Box<dyn std::error::Error>> {
    let input_file = File::open(INPUT_FILENAME)?;
    let reader = BufReader::new(input_file);
    let mut max: u32 = 0;
    let mut calorie_count: u32 = 0;

    for line in reader.lines() {
        let calories = line?;

        if calories.is_empty() {
            max = if max > calorie_count {
                max
            } else {
                calorie_count
            };
            calorie_count = 0;
        } else {
            calorie_count += calories.parse::<u32>()?;
        }
    }

    Ok(max)
}

fn part_b() -> Result<u32, Box<dyn std::error::Error>> {
    let input_file = File::open(INPUT_FILENAME)?;
    let reader = BufReader::new(input_file);
    let mut top_calories = BinaryHeap::with_capacity(MAX_LENGTH);
    let mut calorie_count: u32 = 0;

    for line in reader.lines() {
        let calories = line?;

        if calories.is_empty() {
            top_calories.push(calorie_count);
            calorie_count = 0;

            if top_calories.len() > MAX_LENGTH {
                let temp_arr = top_three(&mut top_calories);
                drop(top_calories);
                top_calories = BinaryHeap::from(temp_arr);
            }
        } else {
            calorie_count += calories.parse::<u32>()?;
        }
    }

    if calorie_count != 0 {
        top_calories.push(calorie_count);
    }

    Ok(top_three(&mut top_calories).iter().sum())
}

fn top_three<T>(bh: &mut BinaryHeap<T>) -> [T; MAX_LENGTH]
where
    T: Ord,
{
    [bh.pop().unwrap(), bh.pop().unwrap(), bh.pop().unwrap()]
}
