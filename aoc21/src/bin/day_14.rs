#![feature(iter_map_windows)]
use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/14_full.txt"));

fn parse(input: &str) -> (String, HashMap<(char, char), char>) {
    let (polymer, mappings) = input.split_once("\n\n").unwrap();
    (
        polymer.to_string(),
        mappings
            .lines()
            .map(|line| {
                let key = (line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap());
                let val = line.chars().last().unwrap();
                (key, val)
            })
            .collect(),
    )
}

fn step(polymer: String, map: &HashMap<(char, char), char>) -> String {
    let mut new_string: String = polymer
        .chars()
        .map_windows(|[a, b]| {
            let new = format!("{}{}", a, map.get(&(*a, *b)).unwrap());
            new
        })
        .collect();
    new_string + &polymer.chars().last().unwrap().to_string()
}

fn part_two(input: &str) {}

fn part_one(input: &str) -> usize {
    let (mut polymer, mapping) = parse(input);
    for s in 0..10 {
        polymer = step(polymer, &mapping);
    }
    let mut occurences: HashMap<char, usize> = HashMap::new();
    for c in polymer.chars() {
        match occurences.get(&c) {
            Some(n) => {
                occurences.insert(c, n + 1);
            }
            None => {
                occurences.insert(c, 1);
            }
        }
    }
    let mut ocs: Vec<usize> = occurences.values().map(|x| *x).collect();
    ocs.sort();
    ocs.last().unwrap() - ocs[0]
}
fn main() {
    println!("1: {}", part_one(INPUT));
    println!("2:");
    part_two(INPUT);
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/14_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 1588);
        assert_eq!(part_one(INPUT), 2703);
    }
}
