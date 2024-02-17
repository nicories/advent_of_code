const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_full.txt"));

type Grid = Vec<Vec<u32>>;
fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}
fn part_two(input: &str) -> u32 {
    0
}
fn part_one(input: &str) -> u32 {
    let data = parse(input);
    let mut sum = 0;
    let row_length = data[0].len();
    let column_length = data.len();
    for x in 0..row_length {
        for y in 0..column_length {
            let number = data[y][x];
            // left
            if x != 0 {
                if number >= data[y][x - 1] {
                    continue;
                }
            }
            // right
            if x != row_length - 1 {
                if number >= data[y][x + 1] {
                    continue;
                }
            }
            // up
            if y != 0 {
                if number >= data[y - 1][x] {
                    continue;
                }
            }
            // down
            if y != column_length - 1 {
                if number >= data[y + 1][x] {
                    continue;
                }
            }
            sum += number + 1;
        }
    }
    sum
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 15);
        assert_eq!(part_one(INPUT), 575);

        // assert_eq!(part_two(INPUT_TEST), 5353);
        // assert_eq!(part_two(INPUT), 1040429);
    }
}
