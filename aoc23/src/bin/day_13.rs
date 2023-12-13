const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/13_full.txt"));

fn transpose_lines(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let num_lines = lines.len();

    let num_columns = lines[0].chars().count();
    let mut result = String::with_capacity(num_lines * (num_columns + 1));

    for col in 0..num_columns {
        for line in &lines {
            if let Some(ch) = line.chars().nth(col) {
                result.push(ch);
            }
        }
        result.push('\n');
    }
    result
}

fn reflections_line(line: &str) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for i in 0..line.len() - 1 {
        if line.chars().nth(i) == line.chars().nth(i + 1) {
            let mut length = 1;
            let mut is_reflection = true;
            let missing_chars = if line.len() - i - 2 < i {
                line.len() - i - 2
            } else {
                i
            };
            for j in 0..missing_chars {
                length += 1;
                let c_left = line.chars().nth(i - 1 - j).unwrap();
                let c_right = line.chars().nth(i + 2 + j).unwrap();
                if c_left != c_right {
                    is_reflection = false;
                    break;
                }
            }
            if is_reflection {
                v.push((i, length));
            }
        }
    }
    v
}

fn block_value(input: &str) -> usize {
    let mut sum = 0;
    let horizontal_reflections: Vec<Vec<(usize, usize)>> =
        input.lines().map(|line| reflections_line(line)).collect();
    for v in &horizontal_reflections {
        if let Some(x) = v
            .iter()
            .find(|x| horizontal_reflections.iter().all(|z| z.contains(&x)))
        {
            sum += x.0 + 1;
            break;
        }
    }
    let vertical_lines = transpose_lines(input);
    let vertical_reflections: Vec<Vec<(usize, usize)>> = vertical_lines
        .lines()
        .map(|line| reflections_line(line))
        .collect();
    for v in &vertical_reflections {
        if let Some(x) = v
            .iter()
            .find(|x| vertical_reflections.iter().all(|z| z.contains(&x)))
        {
            sum += (x.0 + 1) * 100;
            break;
        }
    }
    sum
}

fn parse(input: &str) -> usize {
    input.split("\n\n").map(|block| block_value(block)).sum()
}

fn part_two(input: &str) -> usize {
    parse(input)
}

fn part_one(input: &str) -> usize {
    parse(input)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/13_test.txt"));

    #[test]
    fn test() {
        // assert_eq!(reflections_line("#.##..##."), vec![(4, 4), (6, 2)]);
        // assert_eq!(reflections_line("..#.##.#."), vec![(0, 1), (4, 4)]);
        // assert_eq!(reflections_line("..##..."), vec![(0, 1), (2, 3), (5, 1)]);
        // assert_eq!(block_value(INPUT_TEST.split("\n\n").nth(0).unwrap()), 5);
        // assert_eq!(block_value(INPUT_TEST.split("\n\n").nth(1).unwrap()), 400);

        assert_eq!(part_one(INPUT_TEST), 405);
        // assert_eq!(part_one(INPUT), 9647174);
        // assert_eq!(galaxy_paths(INPUT_TEST, 10), 1030);
        // assert_eq!(galaxy_paths(INPUT_TEST, 100), 8410);
        // assert_eq!(part_two(INPUT), 377318892554);
    }
}
