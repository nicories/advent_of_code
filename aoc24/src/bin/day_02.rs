const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/02_full.txt"));

type Report = Vec<i32>;

fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}
fn is_safe(report: &Report) -> bool {
    let (min_diff, max_diff) = if report[1] > report[0] {
        (1, 3)
    } else {
        (-3, -1)
    };
    let diffs: Vec<i32> = report.windows(2).map(|pair| pair[1] - pair[0]).collect();
    if diffs
        .iter()
        .any(|diff| *diff < min_diff || *diff > max_diff)
    {
        return false;
    }
    return true;
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let reports = parse(input);
    reports.iter().filter(|report| is_safe(report)).count()
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
        assert_eq!(part_one(INPUT_TEST), 2);
        assert_eq!(part_one(INPUT), 371);

        // assert_eq!(part_two(INPUT_TEST), 31);
        // assert_eq!(part_two(INPUT), 24869388);
    }
}
