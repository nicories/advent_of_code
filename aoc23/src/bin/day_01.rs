const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/01_full.txt"));

fn string_to_digit(s: &str) -> Option<u32> {
    let first = s.chars().nth(0).unwrap();
    if let Some(digit) = first.to_digit(10) {
        return Some(digit);
    }
    if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn part_two(input: &str) -> u32 {
    let mut v = vec![];
    for line in input.lines() {
        let mut line_digits = vec![];
        for i in 0..line.len() {
            if let Some(d) = string_to_digit(line.split_at(i).1) {
                line_digits.push(d);
            }
        }
        v.push(line_digits);
    }
    v.iter()
        .map(|v| v.first().unwrap() * 10 + v.last().unwrap())
        .sum()
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .map(|v: Vec<u32>| v.first().unwrap() * 10 + v.last().unwrap())
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
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/01_test2.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 142);
        assert_eq!(part_one(INPUT), 55029);

        assert_eq!(part_two(INPUT_TEST2), 281);
        assert_eq!(part_two(INPUT), 55686);
    }
}
