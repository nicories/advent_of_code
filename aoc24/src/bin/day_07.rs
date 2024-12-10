const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/07_full.txt"));

type Equation = (usize, Vec<usize>);

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (result, rest) = line.split_once(":").unwrap();
            (
                result.parse().unwrap(),
                rest.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn test_equations(input: &str, valid_operations: &[fn(usize, usize) -> usize]) -> usize {
    parse(input)
        .into_iter()
        .filter(|(result, operands)| {
            for n_operator in 0..valid_operations.len().pow(operands.len() as u32 - 1) {
                let mut operands_iter = operands.iter();
                // start with first operand, since it's always plus
                let start = *operands_iter.next().unwrap();
                let final_res = operands_iter.enumerate().fold(start, |res, (i, a)| {
                    let op_index = (n_operator / (valid_operations.len().pow(i as u32)))
                        % valid_operations.len();
                    valid_operations[op_index](res, *a)
                });
                if final_res == *result {
                    return true;
                }
            }
            false
        })
        .map(|(res, _)| res)
        .sum()
}

fn part_two(input: &str) -> usize {
    let ops = vec![|a, b| a + b, |a, b| a * b, |a, b| {
        a * 10_usize.pow((b as f64).log10().floor() as u32 + 1) + b
    }];
    test_equations(input, &ops)
}

fn part_one(input: &str) -> usize {
    let ops = vec![|a, b| a + b, |a, b| a * b];
    test_equations(input, &ops)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/07_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 3749);
        assert_eq!(part_one(INPUT), 5702958180383);

        assert_eq!(part_two(INPUT_TEST), 11387);
        assert_eq!(part_two(INPUT), 92612386119138);
    }
}
