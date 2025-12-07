use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/07_full.txt"));

#[derive(Debug)]
enum Tile {
    Empty,
    Start,
    Splitter,
}
struct Grid {
    tiles: Vec<Vec<Tile>>,
    start_x: usize,
}
fn parse(input: &str) -> Grid {
    let mut start_x = 0;
    let tiles = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    'S' => {
                        start_x = x;
                        Tile::Start
                    }
                    '^' => Tile::Splitter,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect();
    Grid { tiles, start_x }
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let grid = parse(input);
    let mut positions = vec![(grid.start_x, 0)];
    let mut encountered_splitters = HashSet::new();

    while let Some(position) = positions.pop() {
        let next_pos = (position.0, position.1 + 1);
        if next_pos.1 == grid.tiles.len() {
            continue;
        }
        match grid.tiles[next_pos.1][next_pos.0] {
            Tile::Empty => positions.push(next_pos),
            Tile::Splitter => {
                if encountered_splitters.contains(&next_pos) {
                    continue;
                }
                encountered_splitters.insert(next_pos);
                let left = (next_pos.0 - 1, next_pos.1);
                let right = (next_pos.0 + 1, next_pos.1);
                positions.push(left);
                positions.push(right);
            }
            Tile::Start => panic!("at the disco"),
        }
    }
    encountered_splitters.len()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/07_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 21);
        assert_eq!(part_one(INPUT), 1590);

        // assert_eq!(part_two(INPUT_TEST), 3263827);
        // assert_eq!(part_two(INPUT), 11643736116335);
    }
}
