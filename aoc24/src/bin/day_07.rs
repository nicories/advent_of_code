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

fn equation_is_valid((result, operands): &Equation) -> bool {
    let n_operators = operands.len() - 1;
    for n_operator in 0..2_i32.pow(n_operators as u32) {
        let mut operands_iter = operands.iter();
        let start_result = operands_iter.next().unwrap().clone();
        let final_res = operands_iter.enumerate().fold(start_result, |res, (i, a)| {
            if (n_operator & 1 << i) >> i == 0 {
                res + a
            } else {
                res * a
            }
        });
        if final_res == *result {
            return true;
        }
    }
    false
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(equation_is_valid)
        .map(|(res, _)| res)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/07_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 3749);
        assert_eq!(part_one(INPUT), 5702958180383);

        // assert_eq!(part_two(INPUT_TEST), 6);
        // assert_eq!(part_two(INPUT), 1909);
    }
}
