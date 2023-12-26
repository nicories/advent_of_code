// the part 2 solution is simply awful but couldn't think of anything better

use std::{collections::HashMap, ops::Index};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/12_full.txt"));

#[derive(Debug, PartialEq, Clone)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}
impl Spring {
    pub fn from_c(c: &char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("unexpected char"),
        }
    }
}
struct SpringRow {
    row: Vec<Spring>,
    damaged_groups: Vec<usize>,
}
impl SpringRow {
    pub fn count_arrangements(
        &self,
        index: usize,
        group_index: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if group_index == self.damaged_groups.len() {
            if index < self.row.len() && self.row[index..].contains(&Spring::Damaged) {
                return 0;
            } else {
                return 1;
            }
        }
        if index >= self.row.len() {
            return 0;
        }
        let group_length = self.damaged_groups[group_index];
        if index + group_length > self.row.len() {
            return 0;
        }
        let next_index = match self.row[index..]
            .iter()
            .position(|x| matches!(x, Spring::Damaged | Spring::Unknown))
        {
            Some(i) => index + i,
            None => return 0,
        };
        if next_index + group_length > self.row.len() {
            return 0;
        }
        if let Some(res) = cache.get(&(next_index, group_index)) {
            return *res;
        }
        let mut result = 0;

        let count_damaged = self.row[next_index..next_index + group_length]
            .iter()
            .filter(|x| matches!(x, Spring::Damaged))
            .count();
        let count_unknown = self.row[next_index..next_index + group_length]
            .iter()
            .filter(|x| matches!(x, Spring::Unknown))
            .count();
        if count_damaged <= group_length && count_damaged + count_unknown >= group_length {
            if !(next_index + group_length < self.row.len()
                && self.row[next_index + group_length] == Spring::Damaged)
            {
                result +=
                    self.count_arrangements(next_index + group_length + 1, group_index + 1, cache);
            }
        }
        if matches!(self.row[next_index], Spring::Unknown) {
            result += self.count_arrangements(next_index + 1, group_index, cache);
        }
        cache.insert((next_index, group_index), result);
        result
    }
    pub fn unfold(&self) -> Self {
        let mut new_springs = self.row.clone();
        let mut new_groups = self.damaged_groups.clone();
        for i in 0..4 {
            if i < 4 {
                new_springs.push(Spring::Unknown);
            }
            new_springs.extend(self.row.clone());
            new_groups.extend(self.damaged_groups.clone());
        }
        Self {
            row: new_springs,
            damaged_groups: new_groups,
        }
    }
}
fn parse(input: &str) -> Vec<SpringRow> {
    let mut v = vec![];
    for line in input.lines() {
        let (springs_split, groups_split) = line.split_once(' ').unwrap();
        let row = springs_split.chars().map(|c| Spring::from_c(&c)).collect();
        let damaged_groups = groups_split
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        v.push(SpringRow {
            row,
            damaged_groups,
        })
    }
    v
}

fn part_two(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|row| row.unfold())
        .map(|row| row.count_arrangements(0, 0, &mut HashMap::new()))
        .sum()
}

fn part_one(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|row| row.count_arrangements(0, 0, &mut HashMap::new()))
        .sum()
}
fn main() {
    println!("1: {}", part_one(INPUT));
    println!("2: {}", part_two(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/12_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 21);
        assert_eq!(part_one(INPUT), 7460);
        let row = SpringRow {
            row: vec![Spring::Operational, Spring::Damaged],
            damaged_groups: vec![1],
        };
        let unfolded_row = row.unfold();
        assert_eq!(
            unfolded_row.row,
            vec![
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged
            ]
        );

        assert_eq!(part_two(INPUT_TEST), 525152);
        assert_eq!(part_two(INPUT), 6720660274964);
    }
}
