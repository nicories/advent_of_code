use std::ops::RangeInclusive;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_full.txt"));

fn parse(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once("-").unwrap();
            RangeInclusive::new(start_str.parse().unwrap(), end_str.parse().unwrap())
        })
        .collect();
    let ingredients = ingredients_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (ranges, ingredients)
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let (ranges, ingredients) = parse(input);
    ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(&ingredient)))
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 3);
        assert_eq!(part_one(INPUT), 726);

        // assert_eq!(part_two(INPUT_TEST), 43);
        // assert_eq!(part_two(INPUT), 9050);
    }
}
