use std::fs::read_to_string;

pub fn run_day5(path: &str) -> u32 {
    let input_string = read_to_string(path).unwrap();
    let polymer: Vec<char> = input_string.chars().collect();

    let result = reduce_polymer(polymer);
    return result;
}

pub fn run_day5_part2(path: &str) -> u32 {
    let input_string = read_to_string(path).unwrap();
    let polymer: Vec<char> = input_string.chars().collect();

    let alphabet: Vec<char> = ('a'..='z').into_iter().collect::<Vec<char>>();
    // let mut reduced_sets: HashMap<char, i32> = HashMap::new();
    let mut shortest_length: u32 = u32::MAX;

    for char in alphabet {
        let mut improved_polymer = polymer.clone();
        improved_polymer.retain(|&x| x != char && x != char.to_ascii_uppercase());
        let reduced_polymer_size = reduce_polymer(improved_polymer);
        if reduced_polymer_size < shortest_length {
            shortest_length = reduced_polymer_size;
        }
    }

    return shortest_length;
}

pub fn reduce_polymer(polymer_input: Vec<char>) -> u32 {
    let mut n = 0;
    let mut polymer: Vec<char> = polymer_input;
    while n < polymer.len() - 1 {
        let char1 = *polymer.get(n).clone().unwrap();
        let char2 = *polymer.get(n + 1).clone().unwrap();

        if char1 != char2 && char1.to_ascii_lowercase() == char2.to_ascii_lowercase() {
            polymer.remove(n);
            polymer.remove(n);
            if n != 0 {
                n -= 1;
            }
        } else {
            n += 1;
        }
    }
    let result = (polymer.len() as u32) - 1;

    return result;
}

#[cfg(test)]
mod tests {
    use crate::day5::{run_day5, run_day5_part2};

    #[test]
    fn test_day5_part1_with_test_data() {
        let path = "src/input/day5_test_data.txt";
        assert_eq!(run_day5(path), 10);
    }
    #[test]
    fn test_day5_part1_with_real_data() {
        let path = "src/input/day5.txt";
        assert_eq!(run_day5(path), 10250);
    }

    #[test]
    fn test_day5_part2_with_test_data() {
        let path = "src/input/day5_test_data.txt";
        assert_eq!(run_day5_part2(path), 4);
    }
    #[test]
    fn test_day5_part2_with_real_data() {
        let path = "src/input/day5.txt";
        assert_eq!(run_day5_part2(path), 6188);
    }
}
