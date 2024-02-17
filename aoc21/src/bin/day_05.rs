use std::collections::HashMap;

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

fn vent_points(start: Point, end: Point, include_diagonal: bool) -> Vec<Point> {
    if start.1 == end.1 {
        (start.0.min(end.0)..=start.0.max(end.0))
            .map(|x| (x, end.1))
            .collect()
    } else if start.0 == end.0 {
        (start.1.min(end.1)..=start.1.max(end.1))
            .map(|y| (end.0, y))
            .collect()
    } else {
        if !include_diagonal {
            return vec![];
        }
        let mut v = vec![];
        let mut p = start;
        loop {
            v.push(p);
            if p == end {
                break;
            }
            if p.0 < end.0 {
                p.0 += 1;
            } else {
                p.0 -= 1;
            }
            if p.1 < end.1 {
                p.1 += 1;
            } else {
                p.1 -= 1;
            }
        }
        return v;
    }
}
fn count_overlapping_vents(input: &str, include_diagonal: bool) -> usize {
    let vents = parse(input);
    let mut map: HashMap<Point, usize> = HashMap::new();
    for vent in vents {
        let points = vent_points(vent.0, vent.1, include_diagonal);
        for p in points {
            match map.get(&p) {
                Some(count) => map.insert(p, count + 1),
                None => map.insert(p, 1),
            };
        }
    }
    map.values().filter(|v| **v > 1).count()
}
fn part_two(input: &str) -> usize {
    count_overlapping_vents(input, true)
}

fn part_one(input: &str) -> usize {
    count_overlapping_vents(input, false)
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

        assert_eq!(part_two(INPUT_TEST), 12);
        assert_eq!(part_two(INPUT), 17193);
    }
}
