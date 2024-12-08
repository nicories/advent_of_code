const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/04_full.txt"));

type Grid = Vec<Vec<char>>;
fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn vec_get_integer<T>(v: &[T], int: i32) -> Option<&T> {
    if int < 0 {
        None
    } else {
        v.get(int as usize)
    }
}

fn part_two(input: &str) -> i32 {
    0
}

fn part_one(input: &str) -> i32 {
    let grid = parse(input);
    let offsets = [
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
    ];
    let chars = "XMAS".chars();
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, _)| (x, y)))
        .map(|(x, y)| {
            offsets
                .iter()
                .filter_map(|[x_offset, y_offset]| {
                    chars
                        .clone()
                        .enumerate()
                        .all(|(index, c)| {
                            vec_get_integer(&grid, y as i32 + index as i32 * y_offset).is_some_and(
                                |row| {
                                    vec_get_integer(row, x as i32 + index as i32 * x_offset)
                                        .is_some_and(|character| *character == c)
                                },
                            )
                        })
                        .then_some(1)
                })
                .sum::<i32>()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/04_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 18);
        assert_eq!(part_one(INPUT), 2521);

        // assert_eq!(part_two(INPUT_TEST), 48);
        // assert_eq!(part_two(INPUT), 74361272);
    }
}
