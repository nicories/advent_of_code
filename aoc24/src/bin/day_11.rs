use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/11_full.txt"));

// (stone, steps) -> length
type Cache = HashMap<(usize, usize), usize>;
fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

fn blink_single(stone: usize, steps: usize, cache: &mut Cache) -> usize {
    if let Some(res) = cache.get(&(stone, steps)) {
        return *res;
    }
    if steps == 0 {
        return 1;
    }
    let result = match stone {
        0 => blink_single(1, steps - 1, cache),
        _ if stone.to_string().len() % 2 == 0 => {
            let stone_string = stone.to_string();
            let (left, right) = stone_string.split_at(stone_string.len() / 2);
            [left, right]
                .iter()
                .map(|&s| blink_single(s.parse().unwrap(), steps - 1, cache))
                .sum()
        }
        _ => blink_single(stone * 2024, steps - 1, cache),
    };
    cache.insert((stone, steps), result);
    result
}

fn simulate(input: &str, steps: usize) -> usize {
    let mut cache = HashMap::new();
    let stones = parse(input);
    stones
        .iter()
        .map(|stone| blink_single(*stone, steps, &mut cache))
        .sum()
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    simulate(input, 25)
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
        assert_eq!(part_one(INPUT_TEST), 55312);
        assert_eq!(part_one(INPUT), 183484);

        // assert_eq!(part_two(INPUT), 1210);
    }
}
