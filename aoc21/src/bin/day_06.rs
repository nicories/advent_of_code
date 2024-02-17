use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_full.txt"));

fn simulate_single_fish(
    start_state: u32,
    days: u32,
    cache: &mut HashMap<(u32, u32), usize>,
) -> usize {
    if days == 0 {
        return 1;
    }
    if cache.contains_key(&(start_state, days)) {
        return *cache.get(&(start_state, days)).unwrap();
    }
    let result = if start_state == 0 {
        simulate_single_fish(6, days - 1, cache) + simulate_single_fish(8, days - 1, cache)
    } else {
        simulate_single_fish(start_state - 1, days - 1, cache)
    };
    cache.insert((start_state, days), result);
    result
}
fn simulate_fishes(input: &str, days: u32) -> usize {
    let mut cache = HashMap::new();
    let fishes: Vec<u32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    fishes
        .iter()
        .map(|fish| simulate_single_fish(*fish, days, &mut cache))
        .sum()
}
fn part_two(input: &str) -> usize {
    simulate_fishes(input, 256)
}
fn part_one(input: &str) -> usize {
    simulate_fishes(input, 80)
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
        assert_eq!(part_one(INPUT_TEST), 5934);
        assert_eq!(part_one(INPUT), 360268);

        assert_eq!(part_two(INPUT_TEST), 26984457539);
        assert_eq!(part_two(INPUT), 1632146183902);
    }
}
