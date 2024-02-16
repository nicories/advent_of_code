const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/02_full.txt"));

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let up: usize = input
        .lines()
        .filter(|s| s.starts_with("up"))
        .filter_map(|s| s[3..].parse::<usize>().ok())
        .sum();
    let down: usize = input
        .lines()
        .filter(|s| s.starts_with("down"))
        .filter_map(|s| s[5..].parse::<usize>().ok())
        .sum();
    let forward: usize = input
        .lines()
        .filter(|s| s.starts_with("forward"))
        .filter_map(|s| s[8..].parse::<usize>().ok())
        .sum();
    (down - up) * forward
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/02_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 150);
        assert_eq!(part_one(INPUT), 1654760);

        // assert_eq!(part_two(INPUT_TEST), 5);
        // assert_eq!(part_two(INPUT), 1538);
    }
}
