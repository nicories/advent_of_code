use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/01_full.txt"));

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip()
}
fn count_unique(mut v: Vec<u32>) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    while let Some(x) = v.pop() {
        let count = 1 + map.get(&x).unwrap_or(&0);
        map.insert(x, count);
    }
    map
}

fn part_two(input: &str) -> u32 {
    let (left, right) = parse(input);
    let left_unique = count_unique(left);
    let right_unique = count_unique(right);

    left_unique
        .iter()
        .map(|(left_key, left_count)| {
            left_count * left_key * right_unique.get(left_key).unwrap_or(&0)
        })
        .sum()
}

fn part_one(input: &str) -> u32 {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .enumerate()
        .map(|(i, left_value)| {
            let right_value = right[i];
            left_value.abs_diff(right_value)
        })
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/01_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 11);
        assert_eq!(part_one(INPUT), 1110981);

        assert_eq!(part_two(INPUT_TEST), 31);
        assert_eq!(part_two(INPUT), 24869388);
    }
}
