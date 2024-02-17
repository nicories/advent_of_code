const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_full.txt"));

fn parse(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split('|').map(|s| {
                s.trim()
                    .split_whitespace()
                    .map(|inner| inner.to_string())
                    .collect()
            });
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

fn part_two(input: &str) -> u32 {
    0
}
fn part_one(input: &str) -> usize {
    let data = parse(input);
    data.iter()
        .map(|d| {
            d.1.iter()
                .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                .count()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_test.txt"));
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_test2.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST2), 26);
        assert_eq!(part_one(INPUT), 488);

        // assert_eq!(part_two(INPUT_TEST), 168);
        // assert_eq!(part_two(INPUT), 1632146183902);
    }
}
