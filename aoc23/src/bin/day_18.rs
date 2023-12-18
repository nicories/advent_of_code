#![feature(iter_map_windows)]

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/18_full.txt"));

type Position = (i64, i64);
fn polygon_area(points: &Vec<Position>) -> f64 {
    let mut area = 0;
    for i in 0..points.len() {
        let a = points[i];
        let b = if i == points.len() - 1 {
            points[0]
        } else {
            points[i + 1]
        };
        area += a.0 * b.1 - a.1 * b.0;
    }
    area.abs() as f64 / 2.0
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Direction {
    pub fn from_c(c: &str) -> Self {
        match c {
            "R" => Self::Right,
            "L" => Self::Left,
            "D" => Self::Down,
            "U" => Self::Up,
            _ => panic!("at the disco"),
        }
    }
    pub fn from_digit(digit: u32) -> Self {
        match digit {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("at the disco"),
        }
    }
}
#[derive(Debug, PartialEq)]
struct Instruction {
    dir: Direction,
    distance: usize,
}
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|split| {
            let mut clone = split.clone();
            let dir = Direction::from_c(clone.next().unwrap());
            let distance = clone.next().unwrap().parse().unwrap();
            Instruction { dir, distance }
        })
        .collect()
}
fn parse_correct_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|split| {
            let string = split.last().unwrap();
            let hex_str = &mut string[2..string.len() - 1].to_string();
            let direction_digit = hex_str.pop().unwrap().to_digit(16).unwrap();
            let dir = Direction::from_digit(direction_digit);
            let distance = usize::from_str_radix(&hex_str, 16).unwrap();

            Instruction { dir, distance }
        })
        .collect()
}
fn lagoon_area(plan: &Vec<Instruction>) -> usize {
    let mut distance = 0;
    let mut points = vec![];
    points.push((0, 0));
    for row in plan {
        let current_position = points.last().unwrap();
        let new_position = match row.dir {
            Direction::Right => (current_position.0 + row.distance as i64, current_position.1),
            Direction::Down => (current_position.0, current_position.1 - row.distance as i64),
            Direction::Left => (current_position.0 - row.distance as i64, current_position.1),
            Direction::Up => (current_position.0, current_position.1 + row.distance as i64),
        };
        distance += row.distance;
        if new_position != (0, 0) {
            points.push(new_position);
        }
    }
    polygon_area(&points) as usize + distance / 2 + 1
}

fn part_two(input: &str) -> usize {
    let plan = parse_correct_instructions(input);
    lagoon_area(&plan)
}

fn part_one(input: &str) -> usize {
    let plan = parse_instructions(input);
    lagoon_area(&plan)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/18_test.txt"));

    #[test]
    fn test() {
        assert_eq!(
            polygon_area(&vec![(1, 6), (3, 1), (7, 2), (4, 4), (8, 5)]),
            16.5
        );
        assert_eq!(part_one(INPUT_TEST), 62);
        assert_eq!(part_one(INPUT), 47139);

        assert_eq!(part_two(INPUT_TEST), 952408144115);
        assert_eq!(part_two(INPUT), 173152345887206);
    }
}
