use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/17_full.txt"));

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Position,
    current_direction: Direction,
    steps_in_direction: usize,
}
type Position = (usize, usize);

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    pub fn possible_successors(&self, grid: &Grid) -> Vec<State> {
        let mut successors = Vec::new();
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if self.current_direction == direction && self.steps_in_direction == 3 {
                continue;
            }
            // skip reverse directions
            match (self.current_direction, direction) {
                (Direction::Right, Direction::Left)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Up, Direction::Down) => {
                    continue;
                }
                _ => {}
            }
            let new_position = direction.add(self.position);
            // skip out of bounds
            if new_position.0 >= grid[0].len() || new_position.1 >= grid.len() {
                continue;
            }
            let new_cost = self.cost + grid[new_position.1][new_position.0];
            let new_steps = if self.current_direction == direction {
                self.steps_in_direction + 1
            } else {
                1
            };
            successors.push(State {
                cost: new_cost,
                position: new_position,
                current_direction: direction,
                steps_in_direction: new_steps,
            })
        }

        successors
    }
}

fn shortest_path(grid: &Grid, start: Position, goal: Position) -> u32 {
    let mut priority_queue = BinaryHeap::new();
    let mut visited: HashSet<(Position, Direction, usize)> = HashSet::new();

    priority_queue.push(State {
        cost: 0,
        position: start,
        current_direction: Direction::Right,
        steps_in_direction: 0,
    });
    priority_queue.push(State {
        cost: 0,
        position: start,
        current_direction: Direction::Down,
        steps_in_direction: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(state) = priority_queue.pop() {
        // Alternatively we could have continued to find all shortest paths
        if state.position == goal {
            return state.cost;
        }
        for successor in state.possible_successors(grid) {
            if !visited.contains(&(
                successor.position,
                successor.current_direction,
                successor.steps_in_direction,
            )) {
                visited.insert((
                    successor.position,
                    successor.current_direction,
                    successor.steps_in_direction,
                ));
                priority_queue.push(successor);
            }
        }
    }

    panic!("at the disco");
}

type Grid = Vec<Vec<u32>>;
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

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> u32 {
    let grid = parse_grid(input);
    shortest_path(&grid, (0, 0), (grid[0].len() - 1, grid.len() - 1))
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/17_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 102);
        assert_eq!(part_one(INPUT), 742);

        // assert_eq!(part_two(INPUT_TEST), 51);
        // assert_eq!(part_two(INPUT), 244199);
    }
}
