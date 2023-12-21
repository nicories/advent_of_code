use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/21_full.txt"));
type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Garden,
    Rock,
    Start,
}
impl Tile {
    pub fn from_c(c: &char) -> Self {
        match c {
            '#' => Self::Rock,
            '.' => Self::Garden,
            'S' => Self::Start,
            _ => panic!("unexpected char"),
        }
    }
    pub fn is_passable(&self) -> bool {
        return !matches!(self, Self::Rock);
    }
}

#[derive(Debug)]
struct Grid {
    // y -> x
    positions: Vec<Vec<Tile>>,
    start_position: Position,
}
impl Grid {
    fn tile_at(&self, pos: &(usize, usize)) -> Tile {
        if pos == &self.start_position {
            Tile::Garden
        } else {
            self.positions[pos.1][pos.0]
        }
    }
    fn start_position(positions: &Vec<Vec<Tile>>) -> Position {
        for y in 0..positions.len() {
            for x in 0..positions[0].len() {
                if matches!(positions[y][x], Tile::Start) {
                    return (x, y);
                }
            }
        }
        panic!("no start");
    }

    pub fn neighbors(&self, current_position: &(usize, usize)) -> Vec<Position> {
        let mut neighbors = vec![];

        if current_position.0 > 0 {
            neighbors.push((current_position.0 - 1, current_position.1));
        }
        if current_position.0 < self.positions[0].len() - 1 {
            neighbors.push((current_position.0 + 1, current_position.1));
        }
        if current_position.1 > 0 {
            neighbors.push((current_position.0, current_position.1 - 1));
        }
        if current_position.1 < self.positions.len() - 1 {
            neighbors.push((current_position.0, current_position.1 + 1));
        }

        neighbors
            .into_iter()
            .filter(|p| self.tile_at(p).is_passable())
            .collect()
    }
    pub fn parse_grid(input: &str) -> Self {
        let mut start = (0, 0);
        let positions = input
            .lines()
            .rev()
            .map(|line| line.chars().map(|c| Tile::from_c(&c)).collect())
            .collect();

        let start_position = Grid::start_position(&positions);
        Self {
            positions,
            start_position,
        }
    }
}
fn reachable_garden_plots(input: &str, steps: usize) -> usize {
    let grid = Grid::parse_grid(input);
    let mut current_positions: Vec<Position> = vec![grid.start_position.clone()];
    for _ in 0..steps {
        let mut new_positions: Vec<Position> = vec![];
        for pos in &current_positions {
            new_positions.append(&mut grid.neighbors(pos));
        }
        new_positions.sort();
        new_positions.dedup_by(|a, b| a == b);
        current_positions = new_positions;
    }
    current_positions.len()
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    reachable_garden_plots(input, 64)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/21_test.txt"));

    #[test]
    fn test() {
        assert_eq!(reachable_garden_plots(INPUT_TEST, 6), 16);
        // assert_eq!(part_one(INPUT_TEST), 32000000);
        // assert_eq!(part_one(INPUT), 711650489);

        // assert_eq!(part_two(INPUT), 219388737656593);
    }
}
