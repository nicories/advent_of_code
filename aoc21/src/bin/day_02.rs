const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/02_full.txt"));

fn part_two(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let (cmd, number) = line.split_once(' ').unwrap();
        let number: usize = number.parse().unwrap();
        match cmd {
            "up" => aim -= number,
            "down" => aim += number,
            "forward" => {
                horizontal += number;
                depth += number * aim
            }
            _ => panic!("at the disco"),
        }
    }
    horizontal * depth
}

fn part_one(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input.lines() {
        let (cmd, number) = line.split_once(' ').unwrap();
        let number: usize = number.parse().unwrap();
        match cmd {
            "up" => depth -= number,
            "down" => depth += number,
            "forward" => horizontal += number,
            _ => panic!("at the disco"),
        }
    }
    horizontal * depth
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
        assert_eq!(part_one(INPUT_TEST), 150);
        assert_eq!(part_one(INPUT), 1654760);

        assert_eq!(part_two(INPUT_TEST), 900);
        assert_eq!(part_two(INPUT), 1956047400);
    }
}
