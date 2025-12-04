use std::ops::RangeInclusive;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/02_full.txt"));

fn id_is_invalid(id: usize) -> bool {
    let id_str = id.to_string();
    let (left, right) = id_str.split_at(id_str.len() / 2);
    left == right
}

fn id_is_invalid_part2(id: usize) -> bool {
    let id_str = id.to_string();

    for chunk_size in 1..=id_str.len() / 2 {
        let mut chunks: Vec<String> = id_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|c| c.iter().collect())
            .collect();
        chunks.dedup();
        if chunks.len() == 1 {
            return true;
        }
    }
    false
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .split(',')
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap()).collect()
        })
        .collect()
}

fn part_two(input: &str) -> usize {
    parse(input)
        .iter()
        .flatten()
        .filter(|x| id_is_invalid_part2(**x))
        .sum()
}

fn part_one(input: &str) -> usize {
    parse(input)
        .iter()
        .flatten()
        .filter(|x| id_is_invalid(**x))
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/02_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 1227775554);
        assert_eq!(part_one(INPUT), 64215794229);

        assert_eq!(part_two(INPUT_TEST), 4174379265);
        assert_eq!(part_two(INPUT), 85513235135);
    }
}
