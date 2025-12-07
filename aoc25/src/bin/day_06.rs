const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_full.txt"));

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}
type Problem = (Operator, Vec<u64>);
fn parse(input: &str) -> Vec<Problem> {
    let mut v = vec![];
    let lines = input.lines();
    let operators_str = lines.clone().last().unwrap();
    for op_str in operators_str.split_whitespace() {
        let op = match op_str {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("at the disco"),
        };
        v.push((op, vec![]));
    }
    for line in lines {
        for (column, operand_str) in line.split_whitespace().enumerate() {
            // try parse to skip operator row
            if let Ok(operand) = operand_str.parse() {
                v[column].1.push(operand);
            }
        }
    }
    v
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> u64 {
    let problems = parse(input);
    problems
        .iter()
        .map(|(operator, operands)| match operator {
            Operator::Add => operands.iter().sum::<u64>(),
            Operator::Multiply => operands.iter().product(),
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 4277556);
        assert_eq!(part_one(INPUT), 4693159084994);

        // assert_eq!(part_two(INPUT_TEST), 14);
        // assert_eq!(part_two(INPUT), 354226555270043);
    }
}
