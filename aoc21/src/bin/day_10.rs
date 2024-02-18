const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_full.txt"));

fn find_corrupted_lines(input: &str) -> (Vec<String>, usize) {
    let mut sum = 0;
    let mut v = vec![];
    for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            if c == '[' || c == '(' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                let last = stack.pop().unwrap();
                if c == ']' && last != '['
                    || c == ')' && last != '('
                    || c == '}' && last != '{'
                    || c == '>' && last != '<'
                {
                    v.push(line.to_string());
                    sum += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!("at the disco"),
                    }
                }
            }
        }
    }
    (v, sum)
}

fn part_two(input: &str) -> usize {
    let corrupted = find_corrupted_lines(input).0;
    let mut scores = vec![];
    for line in input.lines() {
        let mut score = 0;
        if corrupted.contains(&line.to_string()) {
            continue;
        }
        let mut additions = vec![];
        let mut stack = vec![];
        for c in line.chars() {
            if c == '[' || c == '(' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
        while !stack.is_empty() {
            match stack.pop().unwrap() {
                '[' => additions.push(']'),
                '(' => additions.push(')'),
                '<' => additions.push('>'),
                '{' => additions.push('}'),
                _ => panic!("at the disco"),
            }
        }
        for c in additions {
            score *= 5;
            score += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("at the disco"),
            };
        }

        scores.push(score);
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn part_one(input: &str) -> usize {
    find_corrupted_lines(input).1
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 26397);
        assert_eq!(part_one(INPUT), 374061);

        assert_eq!(part_two(INPUT_TEST), 288957);
        assert_eq!(part_two(INPUT), 2116639949);
    }
}
