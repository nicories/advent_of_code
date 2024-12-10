use std::cmp::Ordering;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_full.txt"));

type Rule = (usize, usize);
type Update = Vec<usize>;
fn parse(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let (rules_block, update_block) = input.split_once("\n\n").unwrap();
    let rules = rules_block
        .lines()
        .map(|line| {
            let split = line.split_once("|").unwrap();
            (split.0.parse().unwrap(), split.1.parse().unwrap())
        })
        .collect();
    let updates = update_block
        .lines()
        .map(|line| line.split(',').map(|d| d.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn partition_updates(rules: &[Rule], updates: Vec<Update>) -> (Vec<Update>, Vec<Update>) {
    updates.into_iter().partition(|update| {
        rules.iter().all(|rule| {
            update.iter().position(|&d| d == rule.0).is_none_or(|left| {
                update
                    .iter()
                    .position(|&d| d == rule.1)
                    .is_none_or(|right| left < right)
            })
        })
    })
}

fn part_two(input: &str) -> usize {
    let (rules, updates) = parse(input);
    let (_, mut incorrect_updates) = partition_updates(&rules, updates);
    incorrect_updates
        .iter_mut()
        .map(|update| {
            update.sort_unstable_by(|&left, &right| {
                rules
                    .iter()
                    .find(|&r| r.0 == left && r.1 == right)
                    .map_or(Ordering::Greater, |_| Ordering::Less)
            });
            update[update.len() / 2]
        })
        .sum()
}

fn part_one(input: &str) -> usize {
    let (rules, updates) = parse(input);
    let (correct_updates, _) = partition_updates(&rules, updates);
    correct_updates
        .iter()
        .map(|update| update[update.len() / 2])
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 143);
        assert_eq!(part_one(INPUT), 4281);

        assert_eq!(part_two(INPUT_TEST), 123);
        assert_eq!(part_two(INPUT), 5466);
    }
}
