use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_full.txt"));

type Grid = Vec<Vec<u32>>;
fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}
fn get_lowest_points(grid: &Grid) -> Vec<(usize, usize)> {
    let mut v = vec![];
    let row_length = grid[0].len();
    let column_length = grid.len();
    for x in 0..row_length {
        for y in 0..column_length {
            let number = grid[y][x];
            // left
            if x != 0 && number >= grid[y][x - 1] {
                continue;
            }
            // right
            if x != row_length - 1 && number >= grid[y][x + 1] {
                continue;
            }
            // up
            if y != 0 && number >= grid[y - 1][x] {
                continue;
            }
            // down
            if y != column_length - 1 && number >= grid[y + 1][x] {
                continue;
            }
            v.push((x, y));
        }
    }
    v
}
fn part_two(input: &str) -> usize {
    let grid = parse(input);
    let mut basins = vec![];
    for start in get_lowest_points(&grid) {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = vec![start];
        while let Some(next) = queue.pop() {
            if grid[next.1][next.0] == 9 || visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            // left
            if next.0 != 0 {
                queue.push((next.0 - 1, next.1));
            }
            // right
            if next.0 != grid[0].len() - 1 {
                queue.push((next.0 + 1, next.1));
            }
            // up
            if next.1 != 0 {
                queue.push((next.0, next.1 - 1));
            }
            // down
            if next.1 != grid.len() - 1 {
                queue.push((next.0, next.1 + 1));
            }
        }
        basins.push(visited.len());
    }
    basins.sort();
    basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
}
fn part_one(input: &str) -> u32 {
    let grid = parse(input);
    get_lowest_points(&grid)
        .iter()
        .map(|(x, y)| grid[*y][*x] + 1)
        .sum()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 15);
        assert_eq!(part_one(INPUT), 575);

        assert_eq!(part_two(INPUT_TEST), 1134);
        assert_eq!(part_two(INPUT), 1019700);
    }
}
