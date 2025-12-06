const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/04_full.txt"));

#[derive(Debug)]
enum Tile {
    Paper,
    Nothing,
}
type Grid = Vec<Vec<Tile>>;
fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Tile::Paper,
                    _ => Tile::Nothing,
                })
                .collect()
        })
        .collect()
}

fn get_accessible_papers(grid: &Grid) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    let search_offsets = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            if !matches!(grid[y as usize][x as usize], Tile::Paper) {
                continue;
            }
            let paper_adjacents = search_offsets
                .iter()
                .filter(|(x_offset, y_offset)| {
                    let x_search = x + x_offset;
                    let y_search = y + y_offset;
                    if x_search < 0
                        || x_search > grid.len() as i32 - 1
                        || y_search < 0
                        || y_search > grid.len() as i32 - 1
                    {
                        return false;
                    }
                    matches!(grid[y_search as usize][x_search as usize], Tile::Paper)
                })
                .count();
            if paper_adjacents < 4 {
                coords.push((x as usize, y as usize));
            }
        }
    }
    coords
}

fn part_two(input: &str) -> usize {
    let mut grid = parse(input);
    let mut removed = 0;
    loop {
        let coords = get_accessible_papers(&grid);
        if coords.len() == 0 {
            break;
        }
        for (x, y) in coords {
            grid[y][x] = Tile::Nothing;
            removed += 1;
        }
    }
    removed
}

fn part_one(input: &str) -> usize {
    let grid = parse(input);
    get_accessible_papers(&grid).len()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/04_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 13);
        assert_eq!(part_one(INPUT), 1441);

        assert_eq!(part_two(INPUT_TEST), 43);
        assert_eq!(part_two(INPUT), 9050);
    }
}
