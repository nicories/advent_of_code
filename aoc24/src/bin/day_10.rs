const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_full.txt"));

type Grid = Vec<Vec<usize>>;
fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}
fn grid_get(grid: &Grid, x: i32, y: i32) -> Option<usize> {
    if x < 0 || y < 0 {
        return None;
    }
    match grid.get(y as usize) {
        Some(row) => row.get(x as usize).copied(),
        None => None,
    }
}

fn hike_to_9(grid: &Grid, (x_start, y_start): (i32, i32), unique: bool) -> i32 {
    let mut queue: Vec<(i32, i32)> = vec![(x_start, y_start)];
    let mut nines = vec![];
    let inc_map = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    while let Some((current_x, current_y)) = queue.pop() {
        let current_height = grid[current_y as usize][current_x as usize];
        if current_height == 9 {
            nines.push((current_x, current_y));
            continue;
        }
        for (x_offset, y_offset) in &inc_map {
            if let Some(neighbour) = grid_get(grid, current_x + x_offset, current_y + y_offset) {
                if neighbour == current_height + 1 {
                    queue.push((current_x + x_offset, current_y + y_offset));
                }
            }
        }
    }
    nines.sort();
    if unique {
        nines.dedup();
    }
    nines.len() as i32
}

fn part_two(input: &str) -> i32 {
    let grid = parse(input);
    let trailheads: Vec<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &height)| (height == 0).then_some((x as i32, y as i32)))
        })
        .collect();
    trailheads
        .iter()
        .map(|start| hike_to_9(&grid, *start, false))
        .sum()
}

fn part_one(input: &str) -> i32 {
    let grid = parse(input);
    let trailheads: Vec<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &height)| (height == 0).then_some((x as i32, y as i32)))
        })
        .collect();
    trailheads
        .iter()
        .map(|start| hike_to_9(&grid, *start, true))
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 36);
        assert_eq!(part_one(INPUT), 531);

        assert_eq!(part_two(INPUT), 1210);
    }
}
