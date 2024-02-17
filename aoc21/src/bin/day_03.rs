const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_full.txt"));

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> u32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let line_length = input.lines().next().unwrap().len();
    let input_length = input.lines().count();

    for i in 0..line_length {
        let zeroes = input
            .lines()
            .filter(|line| line.chars().nth(i).unwrap() == '0')
            .count();
        if zeroes < input_length / 2 {
            gamma_rate += 2_u32.pow(line_length as u32 - i as u32 - 1);
        } else {
            epsilon_rate += 2_u32.pow(line_length as u32 - i as u32 - 1);
        }
    }
    gamma_rate * epsilon_rate
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
        assert_eq!(part_one(INPUT_TEST), 198);
        assert_eq!(part_one(INPUT), 852500);

        // assert_eq!(part_two(INPUT_TEST), 900);
        // assert_eq!(part_two(INPUT), 1956047400);
    }
}
