const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_full.txt"));

#[derive(PartialEq, Debug)]
struct PartNumber {
    x_start: usize,
    x_end: usize,
    y: usize,
    value: u32,
}
impl PartNumber {
    pub fn border_coords(&self, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];
        let x_low = if self.x_start > 0 {
            self.x_start - 1
        } else {
            0
        };
        // top border
        if self.y > 0 {
            for x in x_low..=(self.x_end + 1).min(x_max) {
                v.push((x, self.y - 1));
            }
        }
        // left
        if self.x_start > 0 {
            v.push((self.x_start - 1, self.y));
        }
        // right
        if self.x_end < x_max {
            v.push((self.x_end + 1, self.y));
        }
        // lower border
        if self.y < y_max {
            for x in x_low..=(self.x_end + 1).min(x_max) {
                v.push((x, self.y + 1));
            }
        }
        v
    }
    pub fn borders_on_symbol(&self, input: &str) -> bool {
        let x_max = input.lines().next().unwrap().len() - 1;
        let y_max = input.lines().count() - 1;
        for cord in self.border_coords(x_max, y_max) {
            let c = input
                .lines()
                .nth(cord.1)
                .unwrap()
                .chars()
                .nth(cord.0)
                .unwrap();
            match c {
                '.' => {}
                _ if c.is_digit(10) => {}
                _ => {
                    return true;
                }
            }
        }
        false
    }
}

fn parse_part_numbers(input: &str) -> Vec<PartNumber> {
    let mut v = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut digit_vec: Vec<u32> = vec![];
        // this code is awful
        for (x, char) in line.chars().enumerate() {
            if let Some(d) = char.to_digit(10) {
                digit_vec.push(d);
                if x == line.len() - 1 {
                    if digit_vec.len() > 0 {
                        let mut number = 0;
                        for (i, d) in digit_vec.iter().rev().enumerate() {
                            number += d * 10_u32.pow(i.try_into().unwrap());
                        }
                        v.push(PartNumber {
                            x_start: x + 1 - digit_vec.len(),
                            x_end: x,
                            y: y,
                            value: number,
                        });
                        digit_vec.clear();
                    }
                }
            } else {
                if digit_vec.len() > 0 {
                    let mut number = 0;
                    for (i, d) in digit_vec.iter().rev().enumerate() {
                        number += d * 10_u32.pow(i.try_into().unwrap());
                    }
                    v.push(PartNumber {
                        x_start: x - digit_vec.len(),
                        x_end: x - 1,
                        y: y,
                        value: number,
                    });
                    digit_vec.clear();
                }
            }
        }
    }
    v
}
fn parse_gears(input: &str) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '*' {
                v.push((x, y));
            }
        }
    }
    v
}

fn part_two(input: &str) -> u32 {
    let gears = parse_gears(input);
    let parts = parse_part_numbers(input);
    let x_max = input.lines().next().unwrap().len() - 1;
    let y_max = input.lines().count() - 1;
    let mut sum = 0;
    for gear in gears {
        let filtered_parts = parts
            .iter()
            .filter(|part| part.border_coords(x_max, y_max).contains(&gear));
        if filtered_parts.clone().count() == 2 {
            sum += filtered_parts.map(|part| part.value).product::<u32>();
        }
    }
    sum
}

fn part_one(input: &str) -> u32 {
    parse_part_numbers(input)
        .iter()
        .filter(|part| part.borders_on_symbol(input))
        .map(|part| part.value)
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
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_test2.txt"));

    #[test]
    fn test() {
        {
            let parts = parse_part_numbers(INPUT_TEST);
            assert_eq!(
                parts[0].border_coords(
                    INPUT_TEST.lines().next().unwrap().len() - 1,
                    INPUT_TEST.lines().count() - 1
                ),
                vec![(3, 0), (0, 1), (1, 1), (2, 1), (3, 1)]
            );
            assert!(parts[0].borders_on_symbol(INPUT_TEST));
            assert!(!parts[1].borders_on_symbol(INPUT_TEST));
            assert!(parts[2].borders_on_symbol(INPUT_TEST));
            assert_eq!(part_one(INPUT_TEST), 4361);
        }
        {
            let parts = parse_part_numbers(INPUT_TEST2);

            assert_eq!(parts[2].value, 10);
            assert_eq!(parts[5].value, 5);
            assert_eq!(parts[5].x_start, 9);
            assert_eq!(parts[5].x_end, 9);
            assert_eq!(
                parts[5].border_coords(
                    INPUT_TEST2.lines().next().unwrap().len() - 1,
                    INPUT_TEST2.lines().count() - 1
                ),
                vec![(8, 2), (9, 2), (8, 3), (8, 4), (9, 4)]
            );
            assert!(parts[5].borders_on_symbol(INPUT_TEST2));
            assert_eq!(part_one(INPUT_TEST2), 4361 + 10 + 5);
        }
        {
            let parts = parse_part_numbers(INPUT);
            assert!(parts[6].borders_on_symbol(INPUT));
        }
        assert_eq!(part_one(INPUT), 531561);

        assert_eq!(part_two(INPUT_TEST), 467835);
        assert_eq!(part_two(INPUT), 83279367);
    }
}
