const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_full.txt"));

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let mut fishes: Vec<u32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    for _day in 0..80 {
        let mut new_fishes = 0;
        for fish in &mut fishes {
            if *fish == 0 {
                *fish = 6;
                new_fishes += 1;
            } else {
                *fish -= 1;
            }
        }
        fishes.append(&mut vec![8; new_fishes]);
    }

    fishes.len()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 5934);
        assert_eq!(part_one(INPUT), 360268);

        // assert_eq!(part_two(INPUT_TEST), 12);
        // assert_eq!(part_two(INPUT), 17193);
    }
}
