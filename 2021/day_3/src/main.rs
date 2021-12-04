use std::ops::BitXor;
use std::u64;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut input = BufReader::new(File::open("input.txt").unwrap()).lines().peekable();
    let col_len = input.peek().unwrap().as_ref().unwrap().len() as u64;
    let signal: Vec<u64> = input
        .map(|x| u64::from_str_radix(&x.unwrap(), 2).unwrap())
        .collect();

    // First Part
    let limit_mask: u64 = (0..col_len).fold(0, |sum, elem| sum + (1 << elem));
    let gamma_rate = (0..(col_len + 1)).rev().fold(0_u64, |gamma_acc, col_i| {
        let mask = 1_u64 << col_i;

        if signal
            .iter()
            .map(|elem| mask & elem != 0)
            .filter(|x| *x)
            .count() as f32
            >= (signal.len() as f32 / 2.0).ceil()
        {
            gamma_acc + mask
        } else {
            gamma_acc
        }
    });
    let epsilon_rate = (!gamma_rate) & limit_mask;

    println!(
        "FIRST PART!\n\nGamma Rate: {}\nEpsilon Rate: {}\nAnswer: {}\n\n",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    // Second Part
    fn find_rate(col_len: u64, input_signal: &[u64], oxygen: bool) -> Option<u64> {
        let mut tmp_vec = input_signal.to_owned();
        let mut col_pos: i64 = (col_len - 1) as i64;

        while tmp_vec.len() > 1 && col_pos >= 0 {
            let mask = 1_u64 << col_pos;
            let keep = tmp_vec
                .iter()
                .map(|elem| mask & elem == mask)
                .filter(|x| *x)
                .count() as f32
                >= (tmp_vec.len() as f32 / 2.0).ceil();

            tmp_vec = tmp_vec
                .iter()
                .filter(|elem| **elem & mask == if keep.bitxor(oxygen) { mask } else { 0 })
                .copied()
                .collect();
            col_pos -= 1;
        }

        tmp_vec.pop()
    }
    let oxygen_rate = find_rate(col_len, &signal, true).unwrap();
    let co2_rate = find_rate(col_len, &signal, false).unwrap();

    println!(
        "SECOND PART!\n\nOxygen Rate: {}\nCO2 Rate: {}\nLife Support: {}",
        oxygen_rate,
        co2_rate,
        oxygen_rate * co2_rate,
    );
}
