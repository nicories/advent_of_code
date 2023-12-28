use std::ops::RangeInclusive;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/22_full.txt"));

#[derive(Debug, Clone, PartialEq)]
struct Brick {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}

fn ranges_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    b.contains(a.start()) || b.contains(a.end()) || a.contains(b.start()) || a.contains(b.end())
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
fn bricks_fall(mut bricks: Vec<Brick>) -> (Vec<Brick>, usize) {
    bricks.sort_by(|a, b| a.z.start().cmp(b.z.start()));
    let mut moves = 0;
    let mut new_bricks: Vec<Brick> = vec![];
    for brick in bricks {
        let x = brick.x.clone();
        let y = brick.y.clone();
        let mut offset = 0;
        let bricks_below = new_bricks
            .iter()
            .filter(|b| b.z.end() < brick.z.start() && ranges_overlap(&b.x, &brick.x) && ranges_overlap(&b.y, &brick.y))
            .map(|b| b.clone())
            ;
        for _ in 1..*brick.z.start() {
            if bricks_below.clone()
                .any(|b| *b.z.end() == brick.z.start() - 1 - offset)
            {
                break;
            } else {
                offset += 1;
            }
        }
        let z = RangeInclusive::new(brick.z.start() - offset, brick.z.end() - offset);
        let new_brick = Brick { x, y, z };
        if new_brick != brick {
            moves += 1;
        }
        new_bricks.push(new_brick)
    }
    (new_bricks, moves)
}

fn count_how_many_would_fall(input: &str) -> Vec<usize> {
    let bricks = parse_bricks(input);
    let (fallen_bricks, _) = bricks_fall(bricks);
    let mut v = vec![];
    for i in 0..fallen_bricks.len() {
        let mut c = fallen_bricks.to_vec();
        c.remove(i);
        v.push(bricks_fall(c).1);
    }
    v
}

fn part_two(input: &str) -> usize {
    count_how_many_would_fall(input).iter().sum()
}

fn part_one(input: &str) -> usize {
    count_how_many_would_fall(input)
        .iter()
        .filter(|x| **x == 0)
        .count()
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
        assert_eq!(part_one(INPUT_TEST), 5);
        assert_eq!(part_one(INPUT), 519);
        assert_eq!(part_two(INPUT_TEST), 7);
        assert_eq!(part_two(INPUT), 109531);
    }
}
