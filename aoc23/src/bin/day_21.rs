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
    let mut queue = VecDeque::new();
    let mut visited: HashMap<Position, usize> = HashMap::new();
    for neighbor in grid.neighbors(&grid.start_position) {
        queue.push_back((neighbor, 1));
    }
    while let Some((pos, cost)) = queue.pop_front() {
        if !visited.contains_key(&pos) {
            visited.insert(pos, cost);
            for neighbor in grid.neighbors(&pos) {
                queue.push_back((neighbor, cost + 1));
            }
        }
    }
    visited
        .values()
        .filter(|p| **p <= steps && **p % 2 == steps % 2)
        .count()
}

fn part_two(input: &str) -> usize {
    let steps = 26501365;
    let grid = Grid::parse_grid(input);
    let mut queue = VecDeque::new();
    let mut visited: HashMap<Position, usize> = HashMap::new();
    for neighbor in grid.neighbors(&grid.start_position) {
        queue.push_back((neighbor, 1));
    }
    while let Some((pos, cost)) = queue.pop_front() {
        if !visited.contains_key(&pos) {
            visited.insert(pos, cost);
            for neighbor in grid.neighbors(&pos) {
                queue.push_back((neighbor, cost + 1));
            }
        }
    }

    // start is exactly in the center of the grid
    let distance_to_edge = grid.positions.len() - grid.start_position.0 - 1;
    let even_rhombus = visited
        .values()
        .filter(|steps| **steps % 2 == 0 && **steps <= distance_to_edge)
        .count();
    let odd_rhombus = visited
        .values()
        .filter(|steps| **steps % 2 == 1 && **steps <= distance_to_edge)
        .count();
    let even_corner_rhombus = visited
        .values()
        .filter(|steps| **steps % 2 == 0 && **steps > distance_to_edge)
        .count();
    let odd_corner_rhombus = visited
        .values()
        .filter(|steps| **steps % 2 == 1 && **steps > distance_to_edge)
        .count();

    // length/width of one rhombus is 131 so after that many steps we are at the center of the next one
    // number of crossed rhombus (=202300)
    let n_grids_in_one_direction = (steps - distance_to_edge) / grid.positions.len();
    let n_even_rhombus = n_grids_in_one_direction * n_grids_in_one_direction;
    let n_odd_rhombus = (n_grids_in_one_direction + 1) * (n_grids_in_one_direction + 1);
    let n_corner_rhombus = n_even_rhombus + n_grids_in_one_direction;

    n_odd_rhombus * odd_rhombus
        + n_even_rhombus * even_rhombus
        + n_corner_rhombus * odd_corner_rhombus
        + n_corner_rhombus * even_corner_rhombus
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
        assert_eq!(part_one(INPUT), 3746);

        assert_eq!(part_two(INPUT), 623540829615589);
    }
}
