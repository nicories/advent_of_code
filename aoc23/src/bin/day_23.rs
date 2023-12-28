use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/23_full.txt"));

#[derive(Debug, Clone, PartialEq)]
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
            Direction::Down => (pos.0, pos.1 - 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Up => (pos.0, pos.1 + 1),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}
impl Tile {
    pub fn from_c(c: char) -> Self {
        match c {
            '#' => Self::Forest,
            '.' => Self::Path,
            '>' => Self::Slope(Direction::Right),
            'v' => Self::Slope(Direction::Up),
            '<' => Self::Slope(Direction::Left),
            '^' => Self::Slope(Direction::Down),
            _ => panic!("at the disco"),
        }
    }
}

type Grid = Vec<Vec<Tile>>;

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from_c).collect())
        .collect()
}
type Position = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
    previous_position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .reverse()
            .then_with(|| self.position.cmp(&other.position))
            .reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    pub fn possible_successors(&self, grid: &Grid) -> Vec<State> {
        let mut successors = vec![];
        let current_tile = &grid[self.position.1][self.position.0];
        match current_tile {
            Tile::Path => {
                let mut directions = vec![];
                if self.position.0 != 0 {
                    directions.push(Direction::Left);
                }
                if self.position.0 != grid[0].len() - 1 {
                    directions.push(Direction::Right);
                }
                if self.position.1 != 0 {
                    directions.push(Direction::Down);
                }
                if self.position.1 != grid.len() - 1 {
                    directions.push(Direction::Up);
                }
                for dir in directions {
                    let new_position = dir.add(self.position);
                    if grid[new_position.1][new_position.0] != Tile::Forest
                        && new_position != self.previous_position
                    {
                        successors.push(State {
                            cost: self.cost + 1,
                            position: new_position,
                            previous_position: self.position,
                        });
                    }
                }
            }
            Tile::Slope(dir) => {
                let new_position = dir.add(self.position);
                if grid[new_position.1][new_position.0] != Tile::Forest
                    && new_position != self.previous_position
                {
                    successors.push(State {
                        cost: self.cost + 1,
                        position: dir.add(self.position),
                        previous_position: self.position,
                    })
                }
            }
            Tile::Forest => panic!("should never be on a forest"),
        }
        // println!("h");
        successors
    }
}

fn longest_path(grid: &Grid) -> usize {
    let start = (1, 0);
    let goal = (grid[0].len() - 2, grid.len() - 1);
    let mut priority_queue = BinaryHeap::new();
    // let mut visited = HashSet::new();
    let mut v = vec![];

    priority_queue.push(State {
        cost: 0,
        position: start,
        previous_position: start,
    });

    while let Some(state) = priority_queue.pop() {
        if state.position == goal {
            v.push(state.cost);
            continue;
        }
        for successor in state.possible_successors(grid) {
            // if !visited.contains(&(successor.position)) {
            // println!("walking to {:?}", successor.position);
            // visited.insert((successor.position));
            priority_queue.push(successor);
            // }
        }
    }

    *v.iter().max().unwrap()
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let grid = parse_grid(input);
    longest_path(&grid)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/23_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 94);
        assert_eq!(part_one(INPUT), 2230);
        // assert_eq!(part_two(INPUT_TEST), 7);
        // assert_eq!(part_two(INPUT), 109531);
    }
}
