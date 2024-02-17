use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_full.txt"));

type Point = (u32, u32);
fn parse(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            let mut split = line
                .split("->")
                .map(|s| s.trim().split(',').map(|s| s.parse::<u32>().unwrap()));
            let mut left = split.next().unwrap();
            let left_point = (left.next().unwrap(), left.next().unwrap());
            let mut right = split.next().unwrap();
            let right_point = (right.next().unwrap(), right.next().unwrap());
            (left_point, right_point)
        })
        .collect()
}
fn part_two(input: &str) -> u32 {
    0
}

fn part_one(input: &str) -> usize {
    let vents = parse(input);
    let mut map: HashMap<Point, usize> = HashMap::new();
    for vent in vents {
        if vent.0 .1 == vent.1 .1 {
            let range = if vent.0 .0 < vent.1 .0 {
                vent.0 .0..=vent.1 .0
            } else {
                vent.1 .0..=vent.0 .0
            };
            for x in range {
                let p = (x, vent.0 .1);
                if map.contains_key(&p) {
                    let count = map.get(&p).unwrap() + 1;
                    map.insert(p, count);
                } else {
                    map.insert(p, 1);
                }
            }
        } else if vent.0 .0 == vent.1 .0 {
            let range = if vent.0 .1 < vent.1 .1 {
                vent.0 .1..=vent.1 .1
            } else {
                vent.1 .1..=vent.0 .1
            };
            for y in range {
                let p = (vent.0 .0, y);
                if map.contains_key(&p) {
                    let count = map.get(&p).unwrap() + 1;
                    map.insert(p, count);
                } else {
                    map.insert(p, 1);
                }
            }
        } else {
        }
    }
    map.values().filter(|v| **v > 1).count()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 5);
        assert_eq!(part_one(INPUT), 5585);

        // assert_eq!(part_two(INPUT_TEST), 1924);
        // assert_eq!(part_two(INPUT), 4590);
    }
}
