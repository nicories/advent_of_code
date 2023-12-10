#![feature(iter_map_windows)]

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_full.txt"));

fn parse_sensor_data(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect()
}
fn extrapolate_data(data: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let mut v = vec![];
    for d in data {
        let mut history = vec![];
        history.push(d.to_owned());
        loop {
            let differences: Vec<i32> = history
                .last()
                .unwrap()
                .iter()
                .map_windows(|[a, b]| **b - **a)
                .collect();
            history.push(differences);
            if !history.last().unwrap().iter().any(|d| *d != 0) {
                break;
            }
        }
        v.push(history);
    }
    for data in &mut v {
        let len = data.len();
        for i in (0..len - 1).rev() {
            {
                // forward
                let left_val = *data[i].last().unwrap();
                let low_val = *data[i + 1].last().unwrap();
                data[i].push(left_val + low_val);
            }
            {
                // backward
                let left_val = *data[i].first().unwrap();
                let low_val = *data[i + 1].first().unwrap();
                data[i].insert(0, left_val - low_val);
            }
        }
    }
    v
}

fn part_two(input: &str) -> i32 {
    let sensor_data = parse_sensor_data(input);
    extrapolate_data(&sensor_data)
        .iter()
        .map(|d| d[0].first().unwrap())
        .sum()
}

fn part_one(input: &str) -> i32 {
    let sensor_data = parse_sensor_data(input);
    extrapolate_data(&sensor_data)
        .iter()
        .map(|d| d[0].last().unwrap())
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 114);
        assert_eq!(part_one(INPUT), 1696140818);

        assert_eq!(part_two(INPUT_TEST), 2);
        assert_eq!(part_two(INPUT), 1152);
    }
}
