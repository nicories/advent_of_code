#![feature(iter_map_windows)]

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_full.txt"));

#[derive(Debug, Clone, Copy)]
enum Tile {
    PipeVertical,
    PipeHorizontal,
    PipeNorthEast,
    PipeNorthWest,
    PipeSouthWest,
    PipeSouthEast,
    Ground,
    Start,
}
impl Tile {
    pub fn from_c(c: &char) -> Self {
        match c {
            '|' => Self::PipeVertical,
            '-' => Self::PipeHorizontal,
            'L' => Self::PipeNorthEast,
            'J' => Self::PipeNorthWest,
            '7' => Self::PipeSouthWest,
            'F' => Self::PipeSouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("unexpected char"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    // y -> x
    positions: Vec<Vec<Tile>>,
}
impl Grid {
    fn tile_at(&self, pos: &(usize, usize)) -> Tile {
        self.positions[pos.1][pos.0]
    }
    pub fn ground_positions(&self) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for y in 0..self.positions.len() {
            for x in 0..self.positions[y].len() {
                if matches!(self.tile_at(&(x, y)), Tile::Ground) {
                    v.push((x, y));
                }
            }
        }
        v
    }
    pub fn parse_grid(input: &str) -> Self {
        let positions = input
            .lines()
            .rev()
            .map(|line| line.chars().map(|c| Tile::from_c(&c)).collect())
            .collect();
        Self { positions }
    }
    pub fn main_loop_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = vec![];
        let start = self.start_position();
        positions.push(start.clone());
        loop {
            let current = *positions.last().unwrap();
            let current_tile = self.tile_at(&current);
            let last = if positions.len() >= 2 {
                *positions.get(positions.len() - 2).unwrap()
            } else {
                current.clone()
            };
            let next = match current_tile {
                Tile::PipeVertical => {
                    if current.1 > last.1 {
                        (current.0, current.1 + 1)
                    } else {
                        (current.0, current.1 - 1)
                    }
                }
                Tile::PipeHorizontal => {
                    if current.0 > last.0 {
                        (current.0 + 1, current.1)
                    } else {
                        (current.0 - 1, current.1)
                    }
                }
                Tile::PipeNorthEast => {
                    if current.1 < last.1 {
                        (current.0 + 1, current.1)
                    } else {
                        (current.0, current.1 + 1)
                    }
                }
                Tile::PipeNorthWest => {
                    if current.1 < last.1 {
                        (current.0 - 1, current.1)
                    } else {
                        (current.0, current.1 + 1)
                    }
                }
                Tile::PipeSouthWest => {
                    if current.1 > last.1 {
                        (current.0 - 1, current.1)
                    } else {
                        (current.0, current.1 - 1)
                    }
                }
                Tile::PipeSouthEast => {
                    if current.1 > last.1 {
                        (current.0 + 1, current.1)
                    } else {
                        (current.0, current.1 - 1)
                    }
                }
                Tile::Ground => {
                    panic!("should never stand on ground!")
                }
                Tile::Start => {
                    let tile_left = if current.0 == 0 {
                        Tile::Ground
                    } else {
                        self.tile_at(&(current.0 - 1, current.1))
                    };
                    let tile_right = self.tile_at(&(current.0 + 1, current.1));
                    let tile_up = self.tile_at(&(current.0, current.1 + 1));
                    // let tile_down = self.tile_at(&(current.0, current.1 - 1));
                    let n = if matches!(
                        tile_left,
                        Tile::PipeHorizontal | Tile::PipeNorthEast | Tile::PipeSouthEast
                    ) {
                        (current.0 - 1, current.1)
                    } else if matches!(
                        tile_right,
                        Tile::PipeHorizontal | Tile::PipeNorthWest | Tile::PipeSouthWest
                    ) {
                        (current.0 + 1, current.1)
                    } else if matches!(
                        tile_up,
                        Tile::PipeVertical | Tile::PipeSouthEast | Tile::PipeSouthWest
                    ) {
                        (current.0, current.1 + 1)
                    } else {
                        (current.0, current.1 - 1)
                    };
                    n
                }
            };
            if next == start {
                break;
            }
            positions.push(next);
        }
        positions
    }
    pub fn start_position(&self) -> (usize, usize) {
        for y in 0..self.positions.len() {
            for x in 0..self.positions[0].len() {
                if matches!(self.positions[y][x], Tile::Start) {
                    return (x, y);
                }
            }
        }
        panic!("no start");
    }
}

fn part_two(input: &str) -> usize {
    let grid = Grid::parse_grid(input);
    let main_loop = grid.main_loop_positions();
    let mut sum = 0;
    let grounds = grid.ground_positions();
    for ground in grounds {
        if main_loop.iter().any(|x| x.0 == ground.0 && x.1 < ground.1) // down check
            && main_loop.iter().any(|x| x.0 == ground.0 && x.1 > ground.1) // up check
            && main_loop.iter().any(|x| x.0 < ground.0 && x.1 == ground.1) // left check
            && main_loop.iter().any(|x| x.0 > ground.0 && x.1 == ground.1) // right check
        {
            sum += 1;
        }
    }
    sum
}

fn part_one(input: &str) -> usize {
    let grid = Grid::parse_grid(input);
    let main_loop = grid.main_loop_positions();
    main_loop.len() / 2
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test.txt"));
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test2.txt"));
    const INPUT_TEST3: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test3.txt"));
    const INPUT_TEST3B: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test3b.txt"));
    const INPUT_TEST4: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test4.txt"));

    #[test]
    fn test() {
        {
            let grid = Grid::parse_grid(INPUT_TEST);
            assert_eq!(grid.start_position(), (1, 3));
            let main_loop = grid.main_loop_positions();
            assert_eq!(main_loop.len(), 8);
            assert_eq!(part_one(INPUT_TEST), 4);
        }
        {
            let grid = Grid::parse_grid(INPUT_TEST2);
            assert_eq!(grid.start_position(), (0, 2));
            assert_eq!(part_one(INPUT_TEST2), 8);
        }
        assert_eq!(part_one(INPUT), 6768);

        assert_eq!(part_two(INPUT_TEST3), 4);
        assert_eq!(part_two(INPUT_TEST3B), 4);
        assert_eq!(part_two(INPUT_TEST4), 10);
        // assert_eq!(part_two(INPUT), 1152);
    }
}
