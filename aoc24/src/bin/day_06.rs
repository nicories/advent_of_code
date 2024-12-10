use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_full.txt"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn rotate_90(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Obstruction,
    Start,
}
type Map = Vec<Vec<Tile>>;
type Position = (i32, i32);
impl std::ops::Add<Position> for Direction {
    type Output = Position;
    fn add(self, pos: Position) -> Position {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
        }
    }
}
fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Obstruction,
                    '^' => Tile::Start,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}
// None -> loop
fn simulate_guard(map: &Map) -> Option<Vec<Position>> {
    let start = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, tile)| {
                matches!(tile, Tile::Start).then_some((x as i32, y as i32))
            })
        })
        .next()
        .unwrap();
    let mut positions = vec![start];
    let mut dir = Direction::Up;
    let mut loop_detector = HashSet::new();
    loop_detector.insert((start, dir));
    loop {
        let pos = positions[positions.len() - 1];
        let next_pos = dir + pos;
        if next_pos.0 < 0
            || next_pos.0 >= map[0].len() as i32
            || next_pos.1 < 0
            || next_pos.1 >= map.len() as i32
        {
            break;
        }
        let next_tile = &map[next_pos.1 as usize][next_pos.0 as usize];
        match next_tile {
            Tile::Empty | Tile::Start => {
                if loop_detector.contains(&(next_pos, dir)) {
                    return None;
                }
                loop_detector.insert((next_pos, dir));
                positions.push(next_pos)
            }
            Tile::Obstruction => dir = dir.rotate_90(),
        }
    }
    Some(positions)
}

fn part_two(input: &str) -> usize {
    let mut map = parse(input);
    let coords: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, _)| map[0].iter().enumerate().map(move |(x, _)| (x, y)))
        .collect();
    coords
        .into_iter()
        .filter(|&(x_obstruction, y_obstruction)| {
            if matches!(map[y_obstruction][x_obstruction], Tile::Start) {
                return false;
            }
            let before = map[y_obstruction][x_obstruction];
            map[y_obstruction][x_obstruction] = Tile::Obstruction;
            let opt = simulate_guard(&map);
            map[y_obstruction][x_obstruction] = before.clone();
            opt.is_none()
        })
        .count()
}

fn part_one(input: &str) -> usize {
    let map = parse(input);
    let mut positions = simulate_guard(&map).unwrap();
    positions.sort();
    positions.dedup();
    positions.len()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 41);
        assert_eq!(part_one(INPUT), 5162);

        assert_eq!(part_two(INPUT_TEST), 6);
        assert_eq!(part_two(INPUT), 1909);
    }
}
