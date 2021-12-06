use std::{collections::LinkedList, fs};

// Naive First Part
fn first_part(input: &str) {
    #[derive(Debug, Clone)]
    struct LaternFish {
        pub new: bool,
        pub days: u8,
    }
    const DAYS_INTERVAL: u8 = 80;

    let mut latern_fish = input
        .chars()
        .filter_map(|elt| elt.to_digit(10).map(|x| x as u8))
        .map(|elt| LaternFish {
            new: false,
            days: elt,
        })
        .collect::<LinkedList<_>>();

    (0..DAYS_INTERVAL).for_each(|_| {
        let mut new_fish: usize = 0;
        for elt in latern_fish.iter_mut() {
            if elt.days == 0 {
                new_fish += 1;
                elt.new = false;
                elt.days = 6;
            } else {
                elt.days -= 1;
            }
        }

        (0..new_fish).for_each(|_| latern_fish.push_back(LaternFish { new: true, days: 8 }));
    });

    println!("First: {}", latern_fish.len());
}

fn second_part(input: &str) {
    const DAYS_INTERVAL: u16 = 256;
    let latern_fish_old = &mut [0; 7];
    let latern_fish_new = &mut [0; 9];

    input
        .chars()
        .filter_map(|elt| elt.to_digit(10).map(|x| x as usize))
        .for_each(|elt| {
                latern_fish_old[elt]+=1;
        });

    for _ in 1..DAYS_INTERVAL + 1 {
        let new_head = latern_fish_old[0] + latern_fish_new[0];

        for cycle in 0..latern_fish_new.len() - 1 {
            if cycle < latern_fish_old.len() - 1 {
                latern_fish_old[cycle] = latern_fish_old[cycle + 1];
            }
            latern_fish_new[cycle] = latern_fish_new[cycle + 1];
        }

        latern_fish_old[latern_fish_old.len() - 1] = new_head;
        latern_fish_new[latern_fish_new.len() - 1] = new_head;
    }

    println!(
        "Second: {}",
        latern_fish_old.iter().chain(latern_fish_new.iter()).sum::<u64>()
    );
}

fn main() {
    let input = &fs::read_to_string("input.txt").unwrap();

    first_part(input);
    second_part(input);
}
