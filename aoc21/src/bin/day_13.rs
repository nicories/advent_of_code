use core::num;
use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/13_full.txt"));

type Graph = HashMap<String, Vec<String>>;

enum Split {
    X(i32),
    Y(i32),
}
fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Split>) {
    let (cords, splits) = input.split_once("\n\n").unwrap();
    (
        cords
            .lines()
            .map(|line| {
                let mut split = line.split(',').map(|s| s.parse::<i32>().unwrap());
                (split.next().unwrap(), split.next().unwrap())
            })
            .collect(),
        splits
            .lines()
            .map(|line| {
                let (rest, number) = line.split_once('=').unwrap();
                let number = number.parse().unwrap();
                if rest.contains("x") {
                    Split::X(number)
                } else {
                    Split::Y(number)
                }
            })
            .collect(),
    )
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let (mut cords, splits) = parse(input);
    let split = &splits[0];
    match split {
        Split::X(n) => {
            for cord in &mut cords {
                if cord.0 > *n {
                    cord.0 -= 2 * (cord.0 - n);
                }
            }
        }
        Split::Y(n) => {
            for cord in &mut cords {
                if cord.1 > *n {
                    cord.1 -= 2 * (cord.1 - n);
                }
            }
        }
    }

    cords.sort();
    cords.dedup();
    cords.len()
}
fn main() {
    println!("1: {}", part_one(INPUT));
    println!("2: {}", part_two(INPUT));
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/13_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 17);
        assert_eq!(part_one(INPUT), 712);

        // assert_eq!(part_two(INPUT_TEST), 36);
        // assert_eq!(part_two(INPUT), 84271);
    }
}
