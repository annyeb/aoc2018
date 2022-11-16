use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run_day1() {
    let mut f = BufReader::new(File::open("src/input/day1.txt").unwrap());

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let frequency_changes: Vec<i32> = s.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut current_frequency: i32 = 0;
    for change_value in frequency_changes.iter() {
        current_frequency += change_value;
    }

    println!("The frequency is: {current_frequency}");
}

pub fn run_day1_part2() {
    let mut f = BufReader::new(File::open("src/input/day1.txt").unwrap());

    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();

    let mut current_frequency: i32 = 0;

    let mut unique_frequencies: HashSet<i32> = HashSet::from([0]);

    for change_value in s.lines().map(|x| x.parse::<i32>().unwrap()).cycle() {
        current_frequency += change_value;

        if unique_frequencies.contains(&current_frequency) {
            println!("The first matching frequency is: {current_frequency}");
            break;
        }
        unique_frequencies.insert(current_frequency);
    }
}
