use std::collections::HashMap;

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
fn extrapolate_data(data: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut v = vec![];
    v.push(data.to_owned());
    loop {
        let mut differences = vec![];
        let target = v.pop().unwrap();
        for i in 0..target.len() - 1 {
            differences.push(target[i + 1] - target[i]);
        }
        v.push(target);
        if differences.iter().find(|d| *d != &0).is_none() {
            v.push(differences);
            break;
        }
        v.push(differences);
    }
    v
}
fn finish_data_forward(data: &mut Vec<Vec<i32>>) {
    let len = data.len();
    data[len - 1].push(0);
    for i in (0..len - 1).rev() {
        let left_val = *data[i].last().unwrap();
        let low_val = *data[i + 1].last().unwrap();
        data[i].push(left_val + low_val);
    }
}
fn finish_data_backwards(data: &mut Vec<Vec<i32>>) {
    let len = data.len();
    data[len - 1].push(0);
    for i in (0..len - 1).rev() {
        let left_val = *data[i].first().unwrap();
        let low_val = *data[i + 1].first().unwrap();
        data[i].insert(0, left_val - low_val);
    }
}

fn part_two(input: &str) -> i32 {
    let sensor_data = parse_sensor_data(input);
    let mut extrapolated_data = vec![];
    for data in sensor_data {
        extrapolated_data.push(extrapolate_data(&data));
    }
    for data in &mut extrapolated_data {
        finish_data_backwards(data);
    }
    extrapolated_data
        .iter()
        .map(|d| d[0].first().unwrap())
        .sum()
}

fn part_one(input: &str) -> i32 {
    let sensor_data = parse_sensor_data(input);
    let mut extrapolated_data = vec![];
    for data in sensor_data {
        extrapolated_data.push(extrapolate_data(&data));
    }
    for data in &mut extrapolated_data {
        finish_data_forward(data);
    }
    extrapolated_data.iter().map(|d| d[0].last().unwrap()).sum()
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
        let sensor_data = parse_sensor_data(INPUT_TEST);
        let mut extrapolated_data = vec![];
        for data in sensor_data {
            extrapolated_data.push(extrapolate_data(&data));
        }
        assert_eq!(extrapolated_data[0][0], vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(extrapolated_data[0][1], vec![3, 3, 3, 3, 3]);
        assert_eq!(extrapolated_data[0][2], vec![0, 0, 0, 0]);
        for data in &mut extrapolated_data {
            finish_data_forward(data);
        }
        dbg!(&extrapolated_data[0]);
        assert_eq!(extrapolated_data[0][0], vec![0, 3, 6, 9, 12, 15, 18]);

        assert_eq!(part_one(INPUT_TEST), 114);
        assert_eq!(part_one(INPUT), 1696140818);

        for data in &mut extrapolated_data {
            finish_data_backwards(data);
        }
        assert_eq!(extrapolated_data[2][0], vec![5, 10, 13, 16, 21, 30, 45, 68]);

        assert_eq!(part_two(INPUT_TEST), 2);
        assert_eq!(part_two(INPUT), 1152);
    }
}
