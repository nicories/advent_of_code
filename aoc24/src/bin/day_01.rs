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

fn part_two(input: &str) -> u32 {
    0
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

        // assert_eq!(part_two(INPUT_TEST2), 281);
        // assert_eq!(part_two(INPUT), 55686);
    }
}
