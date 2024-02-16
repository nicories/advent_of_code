#![feature(iter_map_windows)]
const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/01_full.txt"));

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .map_windows(|[d1, d2, d3, d4]| {
            let a = d1 + d2 + d3;
            let b = d2 + d3 + d4;
            b - a
        })
        .filter(|x| *x > 0)
        .count()
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .map_windows(|[a, b]| b - a)
        .filter(|x| *x > 0)
        .count()
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
        assert_eq!(part_one(INPUT_TEST), 7);
        assert_eq!(part_one(INPUT), 1502);

        assert_eq!(part_two(INPUT_TEST), 5);
        assert_eq!(part_two(INPUT), 1538);
    }
}
