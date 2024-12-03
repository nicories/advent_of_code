const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_full.txt"));

fn filter_enabled(input: &str) -> String {
    let enable_str = "do()";
    let disable_str = "don't()";

    let mut in_s = input.to_string();
    let mut current_enabled = true;
    let mut out_s = String::new();

    loop {
        if current_enabled {
            if let Some(disable_index) = in_s.find(disable_str) {
                out_s.push_str(&in_s[0..disable_index]);
                in_s = in_s[disable_index + disable_str.len()..].to_string();
                current_enabled = false;
            } else {
                // end of file
                out_s.push_str(&in_s[0..]);
                break;
            }
        } else {
            if let Some(enable_index) = in_s.find(enable_str) {
                in_s = in_s[enable_index + 1..].to_string();
                current_enabled = true;
            } else {
                // end of file
                break;
            }
        }
    }
    out_s
}
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

fn part_two(input: &str) -> i32 {
    let filtered_input = filter_enabled(input);
    part_one(&filtered_input)
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
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_test2.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 161);
        assert_eq!(part_one(INPUT), 175615763);

        assert_eq!(part_two(INPUT_TEST2), 48);
        assert_eq!(part_two(INPUT), 74361272);
    }
}
