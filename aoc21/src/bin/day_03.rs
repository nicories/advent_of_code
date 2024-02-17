const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_full.txt"));

fn bit_criteria(input: &str, when_most_zeroes: char, when_most_ones: char) -> u32 {
    let mut input_vec: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut index = 0;
    while input_vec.len() != 1 {
        let zeroes = input_vec
            .iter()
            .filter(|line| line.chars().nth(index).unwrap() == '0')
            .count();
        let keep = if zeroes > input_vec.len() / 2 {
            when_most_zeroes
        } else {
            when_most_ones
        };
        input_vec.retain(|line| line.chars().nth(index).unwrap() == keep);
        index += 1;
    }
    u32::from_str_radix(&input_vec[0], 2).unwrap()
}
fn part_two(input: &str) -> u32 {
    let oxygen_rating = bit_criteria(input, '0', '1');
    let co2_rating = bit_criteria(input, '1', '0');
    oxygen_rating * co2_rating
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

        assert_eq!(part_two(INPUT_TEST), 230);
        assert_eq!(part_two(INPUT), 1007985);
    }
}
