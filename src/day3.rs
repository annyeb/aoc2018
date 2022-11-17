use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{BufWriter, Error, Write};
use std::num::ParseIntError;
use std::ops::Index;
use std::str::FromStr;
use std::{
    fs::File,
    io::{self, Read},
};

struct Claim {
    id: i32,
    left_edge: i32,
    top_edge: i32,
    width: i32,
    height: i32,
}

pub fn run_day3() -> Result<(), Error> {
    /*
    let mut f = BufReader::new(File::open("src/input/day3.txt").unwrap());

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut claims: Vec<Claim> = Vec::new();

    let claims_regex = regex::Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    for line in s.lines() {
        let caps = claims_regex
            .captures(line);

        claims.push(Claim {
            id: caps[1].parse::<i32>()?,
            left_edge: caps[2].parse::<i32>()?,
            top_edge: caps[3].parse::<i32>()?,
            width: caps[4].parse::<i32>()?,
            height: caps[5].parse::<i32>()?,
        });


    }

     dbg!(claims);


    */

    Ok(())
}

//pub fn run_day3_part2() {}
