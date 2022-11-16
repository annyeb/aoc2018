use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{BufWriter, Error, Write};

pub fn run_day4(path:&str) -> u32 {
    let input_string = read_to_string(path).unwrap();
    let mut sorted_schedule: Vec<&str> = input_string.lines().collect();

    sorted_schedule.sort_unstable();

    dbg!(&sorted_schedule);

    let mut guard_sleep_schedule = HashMap::new();
    let mut fell_asleep = None;
    let mut guard_id = None;

    for line in sorted_schedule.iter() {
        let guard_category = get_guard_category(line);
        match guard_category{
            GuardCategory::ShiftBegins(id) => {
                guard_sleep_schedule.entry(id).or_insert_with(||vec![0;60]);
                guard_id = Some(id);
            },
            GuardCategory::FallsAsleep(minute) => {
                fell_asleep = Some(minute);
            },
            GuardCategory::WakesUp(minute) => {
                let mut sleeps = guard_sleep_schedule.get_mut(&guard_id.unwrap()).unwrap();
                for i in fell_asleep.unwrap() .. minute {
                    *sleeps.get_mut(i as usize).unwrap() +=1;
                }
            }
        }
    }

    let (sleepy_guard_id, minutes) = guard_sleep_schedule.into_iter().max_by_key(|(id, minutes)|minutes.iter().sum::<i32>()).unwrap();

    let sleepy_minute = minutes.into_iter().enumerate().max_by_key(|(index, counts)| *counts).unwrap().0;
    let result = sleepy_guard_id * sleepy_minute as u32;

    return result;
}

pub fn run_day4_part2() {
    let mut f = BufReader::new(File::open("src/input/day1.txt").unwrap());

    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();
}

#[derive(Eq, PartialEq, Debug)]
enum GuardCategory {
    ShiftBegins(u32),
    FallsAsleep(u32),
    WakesUp(u32),
}

fn get_guard_category(string: &str) -> GuardCategory {
    if string.contains("falls asleep") {
        let claims_regex = regex::Regex::new(r"00:(\d+)] falls asleep").unwrap();
        let caps = claims_regex
            .captures(string).unwrap();
        return GuardCategory::FallsAsleep(caps[1].parse::<u32>().unwrap());
    } else if string.contains("wakes up") {
        let claims_regex = regex::Regex::new(r"00:(\d+)] wakes up").unwrap();
        let caps = claims_regex
            .captures(string).unwrap();
        return GuardCategory::WakesUp(caps[1].parse::<u32>().unwrap());
    } else if string.contains("begins shift") {
        let claims_regex = regex::Regex::new(r"Guard #(\d+) begins shift").unwrap();
        let caps = claims_regex
            .captures(string).unwrap();
        return GuardCategory::ShiftBegins(caps[1].parse::<u32>().unwrap());
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use crate::day4::{get_guard_category, GuardCategory, run_day4};

    #[test]
    fn test_day4_part1() {
        let path = "src/input/day4_test_data.txt";
        assert_eq!(run_day4(path), 10*24);
    }

    #[test]
    fn test_get_guard_category_can_parse_falls_asleep() {
        assert_eq!(
            get_guard_category("[1518-11-03 00:24] falls asleep"),
            GuardCategory::FallsAsleep(24)
        );
    }

    #[test]
    fn test_get_guard_category_can_parse_wakes_up() {
        assert_eq!(
            get_guard_category("[1518-11-03 00:25] wakes up"),
            GuardCategory::WakesUp(25)
        );
    }
        #[test]
    fn test_get_guard_category_can_parse_shift_begins() {
        assert_eq!(
            get_guard_category("[1518-11-05 00:03] Guard #99 begins shift"),
            GuardCategory::ShiftBegins(99)
        );
    }

}
