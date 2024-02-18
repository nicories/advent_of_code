use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/11_full.txt"));

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let mut number_of_flashes = 0;
    let steps = 100;
    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    for _step in 0..steps {
        // increase each by 1
        for line in &mut grid {
            for octopus in line {
                *octopus += 1;
            }
        }
        let mut flashed_map: HashSet<(usize, usize)> = HashSet::new();
        loop {
            let mut flashes = vec![];
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    if grid[y][x] > 9 && !flashed_map.contains(&(x, y)) {
                        flashes.push((x, y));
                        flashed_map.insert((x, y));
                    }
                }
            }
            // no more flashes -> reset and exit
            if flashes.is_empty() {
                number_of_flashes += flashed_map.len();
                for y in 0..grid.len() {
                    for x in 0..grid[0].len() {
                        if grid[y][x] > 9 {
                            grid[y][x] = 0;
                        }
                    }
                }
                break;
            }
            // increase neighbors
            for flash in flashes {
                if flash.1 > 0 {
                    if flash.0 > 0 {
                        grid[flash.1 - 1][flash.0 - 1] += 1;
                    }
                    grid[flash.1 - 1][flash.0] += 1;
                    if flash.0 < grid[0].len() - 1 {
                        grid[flash.1 - 1][flash.0 + 1] += 1;
                    }
                }

                if flash.0 > 0 {
                    grid[flash.1][flash.0 - 1] += 1;
                }
                if flash.0 < grid[0].len() - 1 {
                    grid[flash.1][flash.0 + 1] += 1;
                }

                if flash.1 < grid.len() - 1 {
                    if flash.0 > 0 {
                        grid[flash.1 + 1][flash.0 - 1] += 1;
                    }
                    grid[flash.1 + 1][flash.0] += 1;
                    if flash.0 < grid[0].len() - 1 {
                        grid[flash.1 + 1][flash.0 + 1] += 1;
                    }
                }
            }
        }
    }
    number_of_flashes
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/11_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 1656);
        assert_eq!(part_one(INPUT), 1665);

        // assert_eq!(part_two(INPUT_TEST), 288957);
        // assert_eq!(part_two(INPUT), 2116639949);
    }
}
