// the part 2 solution is simply awful but couldn't think of anything better

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_full.txt"));

#[derive(Debug, Clone, Copy, PartialEq)]
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
    start_position: (usize, usize),
}
impl Grid {
    fn tile_at(&self, pos: &(usize, usize)) -> Tile {
        if pos == &self.start_position {
            Tile::PipeSouthWest
        } else {
            self.positions[pos.1][pos.0]
        }
    }
    pub fn all_positions(&self) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for y in 0..self.positions.len() {
            for x in 0..self.positions[y].len() {
                v.push((x, y));
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
        let start_position = Grid::start_position(&positions);
        Self {
            positions,
            start_position,
        }
    }
    pub fn main_loop_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = vec![];
        let start = self.start_position;
        positions.push(start);
        loop {
            let current = *positions.last().unwrap();
            let current_tile = self.tile_at(&current);
            let last = if positions.len() >= 2 {
                *positions.get(positions.len() - 2).unwrap()
            } else {
                current
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
                    let tile_up = if current.1 == self.positions.len() - 1 {
                        Tile::Ground
                    } else {
                        self.tile_at(&(current.0, current.1 + 1))
                    };
                    if matches!(
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
                    }
                }
            };
            if next == start {
                break;
            }
            positions.push(next);
        }
        positions
    }
    fn start_position(positions: &Vec<Vec<Tile>>) -> (usize, usize) {
        for y in 0..positions.len() {
            for x in 0..positions[0].len() {
                if matches!(positions[y][x], Tile::Start) {
                    return (x, y);
                }
            }
        }
        panic!("no start");
    }
}

fn count_tiles(tiles: &[Tile], tile_type: Tile) -> usize {
    tiles.iter().filter(|tile| **tile == tile_type).count()
}

fn part_two(input: &str) -> usize {
    let grid = Grid::parse_grid(input);
    let main_loop = grid.main_loop_positions();
    let mut sum = 0;
    let grounds = grid.all_positions();
    let grounds_all: Vec<&(usize, usize)> = grounds
        .iter()
        .filter(|pos| !main_loop.contains(pos))
        .collect();
    let mut grounds_inside = vec![];
    for ground in grounds_all {
        if main_loop.iter().any(|x| x.0 == ground.0 && x.1 < ground.1) // down check
            && main_loop.iter().any(|x| x.0 == ground.0 && x.1 > ground.1) // up check
            && main_loop.iter().any(|x| x.0 < ground.0 && x.1 == ground.1) // left check
            && main_loop.iter().any(|x| x.0 > ground.0 && x.1 == ground.1)
        // right check
        {
            grounds_inside.push(ground);
        }
    }
    for ground in grounds_inside {
        let left_tiles: Vec<Tile> = main_loop
            .iter()
            .filter(|x| x.0 < ground.0 && x.1 == ground.1)
            .map(|x| grid.tile_at(x))
            .collect();
        let up_tiles: Vec<Tile> = main_loop
            .iter()
            .filter(|x| x.0 == ground.0 && x.1 > ground.1)
            .map(|x| grid.tile_at(x))
            .collect();
        let down_tiles: Vec<Tile> = main_loop
            .iter()
            .filter(|x| x.0 == ground.0 && x.1 < ground.1)
            .map(|x| grid.tile_at(x))
            .collect();
        let right_tiles: Vec<Tile> = main_loop
            .iter()
            .filter(|x| x.0 > ground.0 && x.1 == ground.1)
            .map(|x| grid.tile_at(x))
            .collect();
        let left_accross = count_tiles(&left_tiles, Tile::PipeVertical)
            + count_tiles(&left_tiles, Tile::PipeNorthEast)
                .min(count_tiles(&left_tiles, Tile::PipeSouthWest))
            + count_tiles(&left_tiles, Tile::PipeNorthWest)
                .min(count_tiles(&left_tiles, Tile::PipeSouthEast));

        let right_accross = count_tiles(&right_tiles, Tile::PipeVertical)
            + count_tiles(&right_tiles, Tile::PipeNorthEast)
                .min(count_tiles(&right_tiles, Tile::PipeSouthWest))
            + count_tiles(&right_tiles, Tile::PipeNorthWest)
                .min(count_tiles(&right_tiles, Tile::PipeSouthEast));

        let up_accross = count_tiles(&up_tiles, Tile::PipeHorizontal)
            + count_tiles(&up_tiles, Tile::PipeNorthEast)
                .min(count_tiles(&up_tiles, Tile::PipeSouthWest))
            + count_tiles(&up_tiles, Tile::PipeNorthWest)
                .min(count_tiles(&up_tiles, Tile::PipeSouthEast));

        let down_accross = count_tiles(&down_tiles, Tile::PipeHorizontal)
            + count_tiles(&down_tiles, Tile::PipeNorthEast)
                .min(count_tiles(&down_tiles, Tile::PipeSouthWest))
            + count_tiles(&down_tiles, Tile::PipeNorthWest)
                .min(count_tiles(&down_tiles, Tile::PipeSouthEast));

        if left_accross % 2 == 1
            && right_accross % 2 == 1
            && up_accross % 2 == 1
            && down_accross % 2 == 1
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
    const INPUT_TEST5: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test5.txt"));

    #[test]
    fn test() {
        {
            let grid = Grid::parse_grid(INPUT_TEST);
            assert_eq!(grid.start_position, (1, 3));
            let main_loop = grid.main_loop_positions();
            assert_eq!(main_loop.len(), 8);
            assert_eq!(part_one(INPUT_TEST), 4);
        }
        {
            let grid = Grid::parse_grid(INPUT_TEST2);
            assert_eq!(grid.start_position, (0, 2));
            assert_eq!(part_one(INPUT_TEST2), 8);
        }
        assert_eq!(part_one(INPUT), 6768);

        assert_eq!(part_two(INPUT_TEST3), 4);
        assert_eq!(part_two(INPUT_TEST3B), 4);
        assert_eq!(part_two(INPUT_TEST4), 8);
        assert_eq!(part_two(INPUT_TEST5), 10);
        assert_eq!(part_two(INPUT), 351);
    }
}
