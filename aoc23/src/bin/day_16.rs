use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/16_full.txt"));

type Grid = Vec<Vec<char>>;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Direction {
    pub fn add(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Up => (pos.0, pos.1 - 1),
        }
    }
}

fn light_path(
    grid: &Grid,
    startx: usize,
    starty: usize,
    dir: Direction,
    map: &mut HashMap<(usize, usize, Direction), (usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut v = vec![];
    let mut current_position = (startx, starty);
    let mut current_dir = dir;
    loop {
        if map.contains_key(&(current_position.0, current_position.1, current_dir)) {
            return v;
        }
        map.insert(
            (current_position.0, current_position.1, current_dir),
            current_position,
        );
        v.push(current_position);
        let current_tile = grid[current_position.1][current_position.0];

        current_dir = match (current_tile, current_dir) {
            ('.', _) => current_dir,
            ('|', Direction::Down | Direction::Up) => current_dir,
            ('-', Direction::Left | Direction::Right) => current_dir,
            ('|', _) => {
                v.append(&mut light_path(
                    grid,
                    current_position.0,
                    current_position.1 + 1,
                    Direction::Down,
                    map,
                ));
                Direction::Up
            }
            ('-', _) => {
                v.append(&mut light_path(
                    grid,
                    current_position.0 - 1,
                    current_position.1,
                    Direction::Left,
                    map,
                ));
                Direction::Right
            }
            ('/', Direction::Right) => Direction::Up,
            ('/', Direction::Down) => Direction::Left,
            ('/', Direction::Left) => Direction::Down,
            ('/', Direction::Up) => Direction::Right,
            ('\\', Direction::Right) => Direction::Down,
            ('\\', Direction::Down) => Direction::Right,
            ('\\', Direction::Left) => Direction::Up,
            ('\\', Direction::Up) => Direction::Left,
            _ => panic!("at the disco"),
        };
        match current_dir {
            Direction::Left if current_position.0 == 0 => {
                return v;
            }
            Direction::Right if current_position.0 == grid[0].len() - 1 => {
                return v;
            }
            Direction::Up if current_position.1 == 0 => {
                return v;
            }
            Direction::Down if current_position.1 == grid.len() - 1 => {
                return v;
            }
            _ => {}
        };
        current_position = current_dir.add(current_position);
    }
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut map = HashMap::new();
    let path = light_path(&grid, 0, 0, Direction::Right, &mut map);
    let mut values: Vec<(usize, usize)> = map.values().cloned().collect();
    values.sort();
    values.dedup();
    values.len()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/16_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 46);
        assert_eq!(part_one(INPUT), 6921);

        // assert_eq!(part_two(INPUT_TEST), 145);
        // assert_eq!(part_two(INPUT), 244199);
    }
}
