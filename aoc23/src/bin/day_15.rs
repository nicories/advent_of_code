use std::{collections::HashMap, future::IntoFuture};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/15_full.txt"));

fn hash_algorithm(input: &str) -> u32 {
    let mut value = 0;
    for c in input.chars() {
        // ignore newline
        if c == '\n' {
            continue;
        }
        let ascii_code = c as u32;
        value += ascii_code;
        value *= 17;
        value %= 256;
    }
    value
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> u32 {
    input.split(',').map(|x| hash_algorithm(x)).sum()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/15_test.txt"));

    #[test]
    fn test() {
        assert_eq!(hash_algorithm("HASH"), 52);
        assert_eq!(part_one(INPUT_TEST), 1320);
        assert_eq!(part_one(INPUT), 514281);

        // assert_eq!(part_two(INPUT_TEST), 64);
        // assert_eq!(part_two(INPUT), 90176);
    }
}
