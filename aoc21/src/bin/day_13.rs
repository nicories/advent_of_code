const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/13_full.txt"));

#[derive(Clone, Copy)]
enum Split {
    X(i32),
    Y(i32),
}
fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Split>) {
    let (cords, splits) = input.split_once("\n\n").unwrap();
    (
        cords
            .lines()
            .map(|line| {
                let mut split = line.split(',').map(|s| s.parse::<i32>().unwrap());
                (split.next().unwrap(), split.next().unwrap())
            })
            .collect(),
        splits
            .lines()
            .map(|line| {
                let (rest, number) = line.split_once('=').unwrap();
                let number = number.parse().unwrap();
                if rest.contains('x') {
                    Split::X(number)
                } else {
                    Split::Y(number)
                }
            })
            .collect(),
    )
}

fn fold(cords: &mut Vec<(i32, i32)>, split: Split) {
    match split {
        Split::X(n) => {
            for cord in &mut *cords {
                if cord.0 > n {
                    cord.0 -= 2 * (cord.0 - n);
                }
            }
        }
        Split::Y(n) => {
            for cord in &mut *cords {
                if cord.1 > n {
                    cord.1 -= 2 * (cord.1 - n);
                }
            }
        }
    }

    cords.sort();
    cords.dedup();
}

fn part_two(input: &str) {
    let (mut cords, splits) = parse(input);
    for split in splits {
        fold(&mut cords, split);
    }
    let max_x = cords.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = cords.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if cords.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_one(input: &str) -> usize {
    let (mut cords, splits) = parse(input);
    let split = &splits[0];
    fold(&mut cords, *split);
    cords.len()
}
fn main() {
    println!("1: {}", part_one(INPUT));
    println!("2:");
    part_two(INPUT);
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/13_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 17);
        assert_eq!(part_one(INPUT), 712);
    }
}
