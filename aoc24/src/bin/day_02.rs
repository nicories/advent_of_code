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

fn is_safe(report: &Report, tolerance: usize) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let n_inc = diffs.iter().filter(|diff| **diff > 0).count();
    let (min_diff, max_diff) = if n_inc > diffs.len() / 2 {
        (1, 3)
    } else {
        (-3, -1)
    };
    let error_index = diffs
        .iter()
        .position(|diff| *diff < min_diff || *diff > max_diff);
    match error_index {
        Some(index) => {
            if tolerance == 0 {
                return false;
            } else {
                let mut pre_copy = report.clone();
                pre_copy.remove(index);
                let pre = is_safe(&pre_copy, tolerance - 1);

                let mut post_copy = report.clone();
                post_copy.remove(index + 1);
                let post = is_safe(&post_copy, tolerance - 1);

                return pre || post;
            }
        }
        None => {
            return true;
        }
    }
}

fn part_two(input: &str) -> usize {
    let reports = parse(input);
    reports.iter().filter(|report| is_safe(report, 1)).count()
}

fn part_one(input: &str) -> usize {
    let reports = parse(input);
    reports.iter().filter(|report| is_safe(report, 0)).count()
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
        assert_eq!(is_safe(&vec![1, 3, 2], 1), true);
        assert_eq!(part_one(INPUT_TEST), 2);
        assert_eq!(part_one(INPUT), 371);

        assert_eq!(part_two(INPUT_TEST), 4);
        assert_eq!(part_two(INPUT), 426);
    }
}
