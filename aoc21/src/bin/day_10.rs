const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_full.txt"));
fn part_two(input: &str) -> usize {
    0
}
fn part_one(input: &str) -> u32 {
    let mut sum = 0;
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/10_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 26397);
        assert_eq!(part_one(INPUT), 374061);

        // assert_eq!(part_two(INPUT_TEST), 1134);
        // assert_eq!(part_two(INPUT), 1019700);
    }
}
