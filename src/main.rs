use crate::day1::run_day1;
use crate::day1::run_day1_part2;
use crate::day2::run_day2;
use crate::day3::run_day3;
use crate::day4::run_day4;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    run_day1();
    run_day1_part2();
    run_day2();
    println!("Answer to day 4 {}", run_day4("src/input/day4.txt"));
}
