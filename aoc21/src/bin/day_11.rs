use std::collections::HashSet;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/11_full.txt"));

fn check_simultaneous_flash(grid: &Vec<Vec<u32>>) -> bool {
    for line in grid {
        for octopus in line {
            if *octopus != 0 {
                return false;
            }
        }
    }
    true
}

fn simulate_step(grid: &mut Vec<Vec<u32>>) -> usize {
    // increase each by 1
    for line in &mut *grid {
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
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    if grid[y][x] > 9 {
                        grid[y][x] = 0;
                    }
                }
            }
            return flashed_map.len();
        }
        // increase neighbors
        for flash in flashes {
            for y in flash.1.saturating_sub(1)..=usize::min(flash.1 + 1, grid.len() - 1) {
                for x in flash.0.saturating_sub(1)..=usize::min(flash.0 + 1, grid[0].len() - 1) {
                    if x == flash.0 && y == flash.1 {
                        continue;
                    }
                    grid[y][x] += 1;
                }
            }
        }
    }
}

fn part_two(input: &str) -> usize {
    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut step = 0;
    loop {
        step += 1;
        simulate_step(&mut grid);
        if check_simultaneous_flash(&grid) {
            return step;
        }
    }
}

fn part_one(input: &str) -> usize {
    let mut number_of_flashes = 0;
    let steps = 100;
    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    for _step in 0..steps {
        number_of_flashes += simulate_step(&mut grid);
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

        assert_eq!(part_two(INPUT_TEST), 195);
        assert_eq!(part_two(INPUT), 235);
    }
}
