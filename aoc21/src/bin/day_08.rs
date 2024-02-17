use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_full.txt"));

fn parse(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split('|').map(|s| {
                s.split_whitespace()
                    .map(|inner| inner.to_string())
                    .collect()
            });
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

fn segment_to_digit(s: &str, mapping: &HashMap<char, char>) -> usize {
    let mapped_string: String = s.chars().map(|c| mapping.get(&c).unwrap()).collect();
    match mapped_string.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        5 => {
            if mapped_string.contains('e') {
                2
            } else if mapped_string.contains('b') {
                5
            } else {
                3
            }
        }
        6 => {
            if !mapped_string.contains('d') {
                0
            } else if mapped_string.contains('e') {
                6
            } else {
                9
            }
        }
        _ => panic!("at the disco"),
    }
}

fn part_two(input: &str) -> usize {
    let data = parse(input);
    data.iter()
        .map(|line| {
            let mut mapping: HashMap<char, char> = HashMap::new();
            let d1 = line.0.iter().find(|s| s.len() == 2).unwrap();
            let d7 = line.0.iter().find(|s| s.len() == 3).unwrap();
            let d4 = line.0.iter().find(|s| s.len() == 4).unwrap();
            let d2_3_5: Vec<&String> = line.0.iter().filter(|s| s.len() == 5).collect();
            // a = 7 - 1
            let a = d7.chars().find(|c| !d1.contains(*c)).unwrap();
            // 4 - 1 => b or d
            let b_or_d: Vec<char> = d4.chars().filter(|c| !d1.contains(*c)).collect();
            assert!(b_or_d.len() == 2);
            let d = *b_or_d
                .iter()
                .find(|c| d2_3_5.iter().all(|v| v.contains(&c.to_string())))
                .unwrap();
            let b = *b_or_d.iter().find(|c| **c != d).unwrap();

            let d5 = d2_3_5.iter().find(|s| s.contains(&b.to_string())).unwrap();
            // c = 4 - 5
            let c = d4.chars().find(|c| !d5.contains(*c)).unwrap();
            // f = 4 - (b, c, d)
            let f = d4
                .chars()
                .find(|character| character != &b && character != &c && character != &d)
                .unwrap();
            // g = 5 - (a,b,d,f)
            let g = d5
                .chars()
                .find(|character| {
                    character != &a && character != &b && character != &d && character != &f
                })
                .unwrap();

            // e = last
            let mut range = 'a'..='g';
            let e = range
                .find(|character| {
                    character != &a
                        && character != &b
                        && character != &c
                        && character != &d
                        && character != &f
                        && character != &g
                })
                .unwrap();
            mapping.insert(a, 'a');
            mapping.insert(b, 'b');
            mapping.insert(c, 'c');
            mapping.insert(d, 'd');
            mapping.insert(e, 'e');
            mapping.insert(f, 'f');
            mapping.insert(g, 'g');
            let output_digits = &line.1;
            segment_to_digit(&output_digits[0], &mapping) * 1000
                + segment_to_digit(&output_digits[1], &mapping) * 100
                + segment_to_digit(&output_digits[2], &mapping) * 10
                + segment_to_digit(&output_digits[3], &mapping)
        })
        .sum()
}
fn part_one(input: &str) -> usize {
    let data = parse(input);
    data.iter()
        .map(|d| {
            d.1.iter()
                .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                .count()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_test.txt"));
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_test2.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST2), 26);
        assert_eq!(part_one(INPUT), 488);

        assert_eq!(part_two(INPUT_TEST), 5353);
        assert_eq!(part_two(INPUT_TEST2), 61229);
        assert_eq!(part_two(INPUT), 1040429);
    }
}
