const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_full.txt"));

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|pack| {
            let mut highest_index = 0;
            for i in 1..pack.len() - 1 {
                if pack[i] > pack[highest_index] {
                    highest_index = i;
                }
            }
            let mut second_highest_index = highest_index + 1;
            for i in highest_index + 2..pack.len() {
                if pack[i] > pack[second_highest_index] {
                    second_highest_index = i;
                }
            }
            dbg!(pack[highest_index], pack[second_highest_index]);
            pack[highest_index] * 10 + pack[second_highest_index]
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 357);
        assert_eq!(part_one(INPUT), 17166);

        // assert_eq!(part_two(INPUT_TEST), 4174379265);
        // assert_eq!(part_two(INPUT), 85513235135);
    }
}
