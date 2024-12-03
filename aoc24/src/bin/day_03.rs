const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_full.txt"));

fn parse(input: &str) -> Vec<(i32, i32)> {
    let mut s = input.to_string();
    let mut pairs = vec![];
    loop {
        let mul = s.find("mul");
        if mul.is_none() {
            break;
        }
        let mul_index = mul.unwrap();
        s = s[mul_index + 3..].to_string();
        if &s[0..1] != "(" {
            continue;
        }
        let close = s.find(')');
        if let Some(i_close) = close {
            let params = &s[1..i_close];
            let split = params.split_once(',');
            if let Some((left_string, right_string)) = split {
                if let (Ok(left), Ok(right)) = (left_string.parse(), right_string.parse()) {
                    pairs.push((left, right));
                } else {
                    s = s[1..].to_string();
                    continue;
                }
            }
            s = s[i_close + 1..].to_string();
        } else {
            break;
        }
    }
    pairs
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> i32 {
    parse(input).iter().map(|(left, right)| left * right).sum()
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

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 161);
        assert_eq!(part_one(INPUT), 175615763);

        // assert_eq!(part_two(INPUT_TEST), 4);
        // assert_eq!(part_two(INPUT), 426);
    }
}
