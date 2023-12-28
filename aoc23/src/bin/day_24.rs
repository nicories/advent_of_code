use std::{cmp::Ordering, ops::RangeInclusive};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/24_full.txt"));

type Position = (f64, f64, f64);

#[derive(Debug, Clone, PartialEq)]
struct HailStone {
    position: Position,
    velocity: (f64, f64, f64),
}

fn parse_hailstones(input: &str) -> Vec<HailStone> {
    input
        .lines()
        .map(|line| {
            let (position_str, velo_str) = line.split_once('@').unwrap();
            let positions: Vec<f64> = position_str
                .split(',')
                .map(|s| s.trim())
                .map(|s| s.parse().unwrap())
                .collect();
            let velos: Vec<f64> = velo_str
                .split(',')
                .map(|s| s.trim())
                .map(|s| s.parse().unwrap())
                .collect();
            HailStone {
                position: (positions[0], positions[1], positions[2]),
                velocity: (velos[0], velos[1], velos[2]),
            }
        })
        .collect()
}

fn intersection_xy(a: &HailStone, b: &HailStone) -> Option<(f64, f64)> {
    if a.velocity.1 * b.velocity.0 == b.velocity.1 * a.velocity.0 {
        return None;
    }
    let a_m = a.velocity.1 / a.velocity.0;
    let a_b = a.position.1 - a.position.0 * a_m;

    let b_m = b.velocity.1 / b.velocity.0;
    let b_b = b.position.1 - b.position.0 * b_m;

    let x = (b_b - a_b) / (a_m - b_m);
    let y = a_m * x + a_b;
    if a.velocity.0 > 0.0 {
        if x < a.position.0 {
            return None;
        }
    } else {
        if x > a.position.0 {
            return None;
        }
    }

    if b.velocity.0 > 0.0 {
        if x < b.position.0 {
            return None;
        }
    } else {
        if x > b.position.0 {
            return None;
        }
    }

    Some((x, y))
}

fn future_intersections_xy(input: &str, test_area: RangeInclusive<f64>) -> Vec<(f64, f64)> {
    let hailstones = parse_hailstones(input);
    let mut v = vec![];
    for (i, a) in hailstones.iter().enumerate() {
        for b in &hailstones[i + 1..] {
            if let Some(intersection) = intersection_xy(a, b) {
                if test_area.contains(&intersection.0) && test_area.contains(&intersection.1) {
                    v.push(intersection);
                }
            }
        }
    }
    v
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    future_intersections_xy(
        input,
        RangeInclusive::new(200000000000000.0, 400000000000000.0),
    )
    .len()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/24_test.txt"));

    #[test]
    fn test() {
        let hailstones = parse_hailstones(INPUT_TEST);
        assert!(intersection_xy(&hailstones[0], &hailstones[1]).is_some());
        assert!(intersection_xy(&hailstones[0], &hailstones[2]).is_some());
        assert!(intersection_xy(&hailstones[0], &hailstones[3]).is_some());
        assert!(intersection_xy(&hailstones[0], &hailstones[4]).is_none());
        let intersections = future_intersections_xy(INPUT_TEST, RangeInclusive::new(7.0, 27.0));
        assert_eq!(intersections.len(), 2);
        // assert!((intersections[0].0 - 14.3333333333).abs() < f64::EPSILON);
        // assert!((intersections[0].1 - 15.333).abs() < f64::EPSILON);
        // assert!((intersections[1].0 - 11.667).abs() < f64::EPSILON);
        // assert!((intersections[1].1 - 16.667).abs() < f64::EPSILON);
        // assert_eq!(part_one(INPUT_TEST), 2);
        assert_eq!(part_one(INPUT), 14672);
        // assert_eq!(part_two(INPUT_TEST), 154);
        // assert_eq!(part_two(INPUT), 6542);
    }
}
