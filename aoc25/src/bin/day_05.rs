const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_full.txt"));

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once("-").unwrap();
            (start_str.parse().unwrap(), end_str.parse().unwrap())
        })
        .collect();
    let ingredients = ingredients_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (ranges, ingredients)
}

fn part_two(input: &str) -> usize {
    let (mut ranges, _) = parse(input);
    ranges.sort();
    // extend ranges
    for (i, range) in ranges.clone().iter().enumerate() {
        assert!(range.0 <= range.1);
        for (j, other_range) in ranges.clone().iter().enumerate() {
            if i == j {
                continue;
            }
            assert!(other_range.0 <= other_range.1);
            if range.0 < other_range.0 && range.1 >= other_range.0 && range.1 <= other_range.1 {
                // end is inside filter_range
                let change_range = ranges.get_mut(j).unwrap();
                change_range.0 = range.0;
            } else if range.0 >= other_range.0
                && range.0 <= other_range.1
                && range.1 > other_range.1
            {
                // start is inside filter_range
                let change_range = ranges.get_mut(j).unwrap();
                change_range.1 = range.1;
            }
        }
    }
    ranges.sort();
    ranges.dedup();
    let ranges_clone = ranges.clone();
    // remove ranges that are inside other ranges
    ranges.retain(|range| {
        for other_range in &ranges_clone {
            if range != other_range && range.0 >= other_range.0 && range.1 <= other_range.1 {
                return false;
            }
        }
        true
    });
    ranges.iter().map(|range| range.1 - range.0 + 1).sum()
}

fn part_one(input: &str) -> usize {
    let (ranges, ingredients) = parse(input);
    ingredients
        .iter()
        .filter(|ingredient| {
            ranges
                .iter()
                .any(|range| **ingredient >= range.0 && **ingredient <= range.1)
        })
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

        assert_eq!(part_two(INPUT_TEST), 14);
        assert_eq!(part_two(INPUT), 354226555270043);
    }
}
