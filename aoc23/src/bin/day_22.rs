use std::ops::RangeInclusive;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/22_full.txt"));

#[derive(Debug, Clone, PartialEq)]
struct Brick {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}

fn ranges_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    for digit in <RangeInclusive<u32> as Clone>::clone(&a).into_iter() {
        if b.contains(&digit) {
            return true;
        }
    }
    false
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start_digits: Vec<u32> = start.split(',').map(|s| s.parse().unwrap()).collect();
            let end_digits: Vec<u32> = end.split(',').map(|s| s.parse().unwrap()).collect();
            Brick {
                x: RangeInclusive::new(start_digits[0], end_digits[0]),
                y: RangeInclusive::new(start_digits[1], end_digits[1]),
                z: RangeInclusive::new(start_digits[2], end_digits[2]),
            }
        })
        .collect()
}
fn bricks_fall(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by(|a, b| a.z.start().cmp(b.z.start()));
    assert!(bricks[0].z.start() <= bricks[1].z.start());
    let mut new_bricks: Vec<Brick> = vec![];
    for brick in bricks.to_vec().into_iter() {
        let x = brick.x.clone();
        let y = brick.y.clone();
        let mut offset = 0;
        let bricks_below: Vec<Brick> = new_bricks
            .iter()
            .filter(|b| ranges_overlap(&b.x, &brick.x) && ranges_overlap(&b.y, &brick.y))
            .map(|b| b.clone())
            .collect();
        for _ in 1..*brick.z.start() {
            if bricks_below
                .iter()
                .any(|b| *b.z.end() == brick.z.start() - 1 - offset)
            {
                break;
            } else {
                offset += 1;
            }
        }
        let z = RangeInclusive::new(brick.z.start() - offset, brick.z.end() - offset);
        new_bricks.push(Brick { x, y, z })
    }
    new_bricks.sort_by(|a, b| a.z.start().cmp(b.z.start()));
    new_bricks
}

fn safe_bricks(bricks: Vec<Brick>) -> usize {
    let mut sum = 0;
    for brick in bricks.to_vec().into_iter() {
        let bricks_directly_above: Vec<Brick> = bricks
            .iter()
            .filter(|b| ranges_overlap(&b.x, &brick.x) || ranges_overlap(&b.y, &brick.y))
            .filter(|b| *b.z.start() == brick.z.end() + 1)
            .map(|b| b.clone())
            .collect();
        if bricks_directly_above.len() == 0 {
            // no bricks above it
            sum += 1;
            continue;
        }
        if bricks_directly_above.iter().all(|above_brick| {
            let bricks_directly_below: Vec<Brick> = bricks
                .iter()
                .filter(|b| {
                    ranges_overlap(&b.x, &above_brick.x) && ranges_overlap(&b.y, &above_brick.y)
                })
                .filter(|b| *b.z.end() == above_brick.z.start() - 1)
                .filter(|b| **b != brick)
                .map(|b| b.clone())
                .collect();
            bricks_directly_below.len() > 0
        }) {
            sum += 1;
        }
    }
    sum
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let bricks = parse_bricks(input);
    let fallen_bricks = bricks_fall(bricks);

    safe_bricks(fallen_bricks)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/22_test.txt"));

    #[test]
    fn test() {
        let bricks = parse_bricks(INPUT_TEST);
        assert_eq!(
            bricks[0],
            Brick {
                x: RangeInclusive::new(1, 1),
                y: RangeInclusive::new(0, 2),
                z: RangeInclusive::new(1, 1),
            }
        );
        let fallen_bricks = bricks_fall(bricks);
        assert_eq!(
            fallen_bricks[0],
            Brick {
                x: RangeInclusive::new(1, 1),
                y: RangeInclusive::new(0, 2),
                z: RangeInclusive::new(1, 1),
            }
        );
        assert_eq!(
            fallen_bricks[6],
            Brick {
                x: RangeInclusive::new(1, 1),
                y: RangeInclusive::new(1, 1),
                z: RangeInclusive::new(5, 6),
            }
        );
        assert_eq!(part_one(INPUT_TEST), 5);
        assert_eq!(part_one(INPUT), 519);
        // assert_eq!(part_two(INPUT_TEST), 16);
        // assert_eq!(part_two(INPUT_TEST, 100,), 6536);
        // assert_eq!(part_two(INPUT_TEST, 500,), 167004);
        // assert_eq!(part_two(INPUT_TEST, 1000,), 668697);
        // assert_eq!(part_two(INPUT_TEST, 5000,), 16733044);

        // assert_eq!(part_two(INPUT), 219388737656593);
    }
}
