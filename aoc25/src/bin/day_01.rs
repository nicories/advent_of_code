const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/01_full.txt"));

enum Rotation {
    Left,
    Right,
}
fn rotate(position: usize, rotation: &Rotation, value: usize) -> usize {
    let val = value % 100;
    match rotation {
        Rotation::Left => {
            if val > position {
                100 - (val - position)
            } else {
                position - val
            }
        }
        Rotation::Right => {
            if val + position >= 100 {
                val + position - 100
            } else {
                val + position
            }
        }
    }
}

fn parse(input: &str) -> Vec<(Rotation, usize)> {
    input
        .lines()
        .map(|line| {
            let (rot_str, num_str) = line.split_at(1);
            let num = num_str.parse::<usize>().unwrap();
            if rot_str == "L" {
                (Rotation::Left, num)
            } else {
                (Rotation::Right, num)
            }
        })
        .collect()
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let mut position = 50;
    parse(input)
        .iter()
        .map(|(rotation, value)| {
            position = rotate(position, rotation, *value);
            position
        })
        .filter(|x| *x == 0)
        .count()
}
fn main() {
    println!("1: {}", part_one(INPUT));
    // println!("2: {}", part_two(INPUT));
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/01_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 3);
        assert_eq!(part_one(INPUT), 1074);

        // assert_eq!(part_two(INPUT_TEST), 6);
        // assert_eq!(part_two(INPUT), 24869388);
    }
}
