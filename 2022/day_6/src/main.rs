use std::fs;

const INPUT_FILENAME: &str = "input.txt";

fn main() {
    println!("Solution to Part A: {}", part_a().unwrap());
    println!("Solution to Part A: {}", part_b().unwrap());
}

fn part_a() -> Option<usize> {
    let message = fs::read_to_string(INPUT_FILENAME).unwrap().chars().collect::<Vec<char>>();
    let mut sliding_window: [char; 4] = [0 as char; 4];
    let mut count: [usize; 26] = [0; 26];

    // NOTE: Using https://www.asciitable.com/ magic, since these are all lower case letters
    for start_index in 0..(message.len() - sliding_window.len()) {
        let end_index = start_index + sliding_window.len();

        for (window_index, message_index) in (start_index..end_index).enumerate() {
            sliding_window[window_index] = message[message_index];

            let count_index = message[message_index] as usize - 97;

            if count[count_index] >= 1 {
                break;
            } else {
                count[count_index] += 1
            }
        }

        if count.iter().sum::<usize>() >= 4 {
            return Some(end_index);
        } else {
            count = [0; 26];
        }
    }

    None
}

fn part_b() -> Option<usize> {
    let message = fs::read_to_string(INPUT_FILENAME).unwrap().chars().collect::<Vec<char>>();
    let mut sliding_window: [char; 14] = [0 as char; 14];
    let mut count: [usize; 26] = [0; 26];

    // NOTE: Using https://www.asciitable.com/ magic, since these are all lower case letters
    for start_index in 0..(message.len() - sliding_window.len()) {
        let end_index = start_index + sliding_window.len();

        for (window_index, message_index) in (start_index..end_index).enumerate() {
            sliding_window[window_index] = message[message_index];

            let count_index = message[message_index] as usize - 97;

            if count[count_index] >= 1 {
                break;
            } else {
                count[count_index] += 1
            }
        }

        if count.iter().sum::<usize>() >= 14 {
            return Some(end_index);
        } else {
            count = [0; 26];
        }
    }

    None
}
