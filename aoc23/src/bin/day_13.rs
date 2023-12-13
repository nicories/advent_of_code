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
fn common_reflections(block: &str) -> Vec<(usize, usize)> {
    let mut common_reflections = vec![];
    let all_reflections: Vec<Vec<(usize, usize)>> =
        block.lines().map(|line| reflections_line(line)).collect();
    if all_reflections.len() != block.lines().count() {
        return common_reflections;
    }
    let first_reflections = all_reflections.first().unwrap();
    for reflection in first_reflections {
        if all_reflections.iter().all(|x| x.contains(reflection)) {
            common_reflections.push((reflection.0, reflection.1));
        }
    }

    common_reflections
}
fn common_reflections_horizontal(block: &str) -> Vec<(usize, usize)> {
    common_reflections(block)
}
fn common_reflections_vertical(block: &str) -> Vec<(usize, usize)> {
    let vertical_lines_original = transpose_lines(block);
    common_reflections(&vertical_lines_original)
}

fn block_permutations(input: &str) -> Vec<String> {
    let mut v = vec![];
    for (i, c) in input.chars().enumerate() {
        let mut new_string = String::from(input);
        if c == '.' {
            new_string.replace_range(i..=i, "#");
            v.push(new_string);
        } else if c == '#' {
            new_string.replace_range(i..=i, ".");
            v.push(new_string);
        }
    }
    v
}

fn block_value_smudge(input: &str) -> usize {
    let horizontal_original = common_reflections_horizontal(input);
    let vertical_original = common_reflections_vertical(input);
    let permutations = block_permutations(input);
    for perm in &permutations {
        let horizontal = common_reflections_horizontal(&perm);
        let vertical = common_reflections_vertical(&perm);
        if let Some(h) = horizontal
            .iter()
            .filter(|x| !horizontal_original.contains(x))
            .next()
        {
            return h.0 + 1;
        } else if let Some(v) = vertical
            .iter()
            .filter(|x| !vertical_original.contains(x))
            .next()
        {
            return (v.0 + 1) * 100;
        }
    }
    panic!("at the disco");
}

fn block_value(input: &str) -> usize {
    let horizontal_reflections = common_reflections(input);
    let vertical_lines = transpose_lines(input);
    let vertical_reflections = common_reflections(&vertical_lines);
    if let Some(horizontal) = horizontal_reflections.first() {
        return horizontal.0 + 1;
    } else {
        return (vertical_reflections.first().unwrap().0 + 1) * 100;
    }
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|block| block_value_smudge(block))
        .sum()
}

fn part_one(input: &str) -> usize {
    input.split("\n\n").map(|block| block_value(block)).sum()
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
        assert_eq!(reflections_line("#.##..##."), vec![(4, 4), (6, 2)]);
        assert_eq!(reflections_line("..#.##.#."), vec![(0, 1), (4, 4)]);
        assert_eq!(reflections_line("..##..."), vec![(0, 1), (2, 3), (5, 1)]);
        assert_eq!(reflections_line("##....#..#."), vec![(0, 1), (7, 3)]);
        assert_eq!(block_value(INPUT_TEST.split("\n\n").nth(0).unwrap()), 5);
        assert_eq!(block_value(INPUT_TEST.split("\n\n").nth(1).unwrap()), 400);

        assert_eq!(part_one(INPUT_TEST), 405);
        assert_eq!(part_one(INPUT), 27505);

        assert_eq!(part_two(INPUT_TEST), 400);
        assert_eq!(part_two(INPUT), 22906);
    }
}
