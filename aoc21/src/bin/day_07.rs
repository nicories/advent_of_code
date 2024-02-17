const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/07_full.txt"));

fn part_two(input: &str) -> u32 {
    let crabs: Vec<u32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut min_fuel = u32::MAX;
    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();
    for horizontal in min_crab..=max_crab {
        min_fuel = min_fuel.min(
            crabs
                .iter()
                .map(|crab| (0..=(crab.max(&horizontal) - crab.min(&horizontal))).sum::<u32>())
                .sum(),
        );
    }
    min_fuel
}
fn part_one(input: &str) -> u32 {
    let crabs: Vec<u32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut min_fuel = u32::MAX;
    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();
    for horizontal in min_crab..=max_crab {
        min_fuel = min_fuel.min(
            crabs
                .iter()
                .map(|crab| crab.max(&horizontal) - crab.min(&horizontal))
                .sum(),
        )
    }
    min_fuel
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
        assert_eq!(part_one(INPUT_TEST), 37);
        assert_eq!(part_one(INPUT), 342534);

        assert_eq!(part_two(INPUT_TEST), 168);
        assert_eq!(part_two(INPUT), 94004208);
    }
}
