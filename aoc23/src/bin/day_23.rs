use std::cmp::Ordering;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/23_full.txt"));

#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
    previous_nodes: Vec<Position>,
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
                let new_positions: Vec<Position> = directions
                    .iter()
                    .filter(|dir| {
                        let new_pos = dir.add(self.position);
                        let tile = &grid[new_pos.1][new_pos.0];
                        if let Tile::Slope(slope_dir) = tile {
                            match (slope_dir, dir) {
                                (Direction::Right, Direction::Left) => return false,
                                (Direction::Down, Direction::Up) => return false,
                                (Direction::Left, Direction::Right) => return false,
                                (Direction::Up, Direction::Down) => return false,
                                _ => return true,
                            };
                        }
                        true
                    })
                    .map(|dir| dir.add(self.position))
                    .filter(|pos| grid[pos.1][pos.0] != Tile::Forest)
                    .filter(|pos| {
                        *pos != self.previous_position && !self.previous_nodes.contains(pos)
                    })
                    .collect();
                if new_positions.is_empty() {
                    // println!("no directions at {:?}", self.position);
                }
                for new_position in &new_positions {
                    let new_nodes = if new_positions.len() == 1 {
                        // println!("only one direction at {:?}", self.position);
                        self.previous_nodes.clone()
                    } else {
                        // println!("found node at {:?}", self.position);
                        let mut c = self.previous_nodes.clone();
                        c.push(self.position);
                        c
                    };
                    successors.push(State {
                        cost: self.cost + 1,
                        position: *new_position,
                        previous_position: self.position,
                        previous_nodes: new_nodes,
                    });
                }
            }
            Tile::Slope(dir) => {
                let new_position = dir.add(self.position);
                if grid[new_position.1][new_position.0] != Tile::Forest {
                    successors.push(State {
                        cost: self.cost + 1,
                        position: new_position,
                        previous_position: self.position,
                        previous_nodes: self.previous_nodes.clone(),
                    })
                }
            }
            Tile::Forest => panic!("should never be on a forest"),
        }
        // println!("pushing {:?} successors", &successors);
        successors
    }
}

fn longest_path(grid: &Grid) -> usize {
    let start = (1, 0);
    let goal = (grid[0].len() - 2, grid.len() - 1);
    let mut queue = vec![];
    let mut v = vec![];

    queue.push(State {
        cost: 0,
        position: start,
        previous_position: start,
        previous_nodes: vec![],
    });

    while let Some(state) = queue.pop() {
        if state.position == goal {
            println!(
                "reached goal after {}, still have {} in queue",
                state.cost,
                queue.len()
            );
            v.push(state.cost);
            continue;
        }
        for successor in state.possible_successors(grid) {
            if queue
                .iter()
                .any(|s| s.position == successor.position && s.cost > successor.cost)
            {
                queue.push(successor);
            }
        }
    }

    *v.iter().max().unwrap()
}

fn part_two(input: &str) -> usize {
    let mut grid = parse_grid(input);
    for line in &mut grid {
        for tile in line {
            if matches!(tile, Tile::Slope(_)) {
                *tile = Tile::Path;
            }
        }
    }

    longest_path(&grid)
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
        assert_eq!(part_two(INPUT_TEST), 154);
        assert_eq!(part_two(INPUT), 6542);
    }
}
