// use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run_day2() {
    let mut f = BufReader::new(File::open("src/input/day2.txt").unwrap());

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let scrambled_letters: Vec<String> = s.lines().map(|x| x.parse::<String>().unwrap()).collect();
    let mut appears_twice = 0;
    let mut appears_thrice = 0;
    for word in scrambled_letters.iter() {
        let mut times_appeared = 0;
        let mut appeared_twice = false;
        let mut appeared_thrice = false;
        for letter in word.chars() {
            times_appeared = word.matches(letter).count();
            if times_appeared == 2 {
                appeared_twice = true;
            }
            if times_appeared == 3 {
                appeared_thrice = true;
            }
        }
        if appeared_twice {
            appears_twice += 1;
        }
        if appeared_thrice {
            appears_thrice += 1;
        }
    }
    // dbg!(scrambled_letters);
    let checksum = appears_twice * appears_thrice;
    println!("Checksum is {checksum}");
}

/*pub fn run_day2_part2() {
    let mut f = BufReader::new(File::open("src/input/day2.txt").unwrap());

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
}*/
