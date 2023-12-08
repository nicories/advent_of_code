use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_full.txt"));

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
}
impl Direction {
    pub fn from_char(c: &char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("lol"),
        }
    }
}
pub fn lcm(nums: &Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}
fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn parse_pouch(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let (direction_block, hash_block) = input.split_once("\n\n").unwrap();
    let directions = direction_block
        .chars()
        .map(|c| Direction::from_char(&c))
        .collect();
    let mut map = HashMap::new();
    for line in hash_block.lines() {
        let (key, pair) = line.split_once('=').unwrap();
        let (left, right) = pair.trim().split_once(',').unwrap();
        let left = left.replace('(', "");
        let right = right.replace(')', "");

        map.insert(
            key.trim().to_string(),
            (left.trim().to_string(), right.trim().to_string()),
        );
    }
    (directions, map)
}

fn count_steps(input: &str, start_pattern: &str, end_pattern: &str) -> u64 {
    let (directions, map) = parse_pouch(input);
    let start_keys: Vec<&String> = map
        .keys()
        .filter(|key| key.ends_with(start_pattern))
        .collect();
    let mut loops: Vec<u64> = vec![];
    for start in start_keys {
        let mut steps = 0;
        let mut current = start;
        let mut directions = directions.iter().cycle();
        loop {
            let dir = directions.next().unwrap();
            current = if dir == &Direction::Left {
                &map.get(current).unwrap().0
            } else {
                &map.get(current).unwrap().1
            };
            steps += 1;
            if current.ends_with(end_pattern) {
                break;
            }
        }
        loops.push(steps);
    }
    lcm(&loops)
}

fn part_two(input: &str) -> u64 {
    count_steps(input, "A", "Z")
}

fn part_one(input: &str) -> u64 {
    count_steps(input, "AAA", "ZZZ")
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_test.txt"));
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_test2.txt"));
    const INPUT_TEST3: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_test3.txt"));

    #[test]
    fn test() {
        let (directions, map) = parse_pouch(INPUT_TEST);
        assert_eq!(directions, vec![Direction::Right, Direction::Left]);
        dbg!(&map);

        assert_eq!(part_one(INPUT_TEST), 2);
        assert_eq!(part_one(INPUT_TEST2), 6);
        assert_eq!(part_one(INPUT), 15517);

        assert_eq!(part_two(INPUT_TEST3), 6);
        assert_eq!(part_two(INPUT), 14935034899483);
    }
}
