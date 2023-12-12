// the part 2 solution is simply awful but couldn't think of anything better

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
    pub fn check_validity(&self) -> bool {
        let mut damaged_counts: Vec<usize> = vec![];
        let mut current_count = 0;
        for spring in &self.row {
            match spring {
                Spring::Damaged => current_count += 1,
                Spring::Operational => {
                    if current_count > 0 {
                        damaged_counts.push(current_count);
                        current_count = 0;
                    }
                }
                Spring::Unknown => {
                    panic!("unexpected unknown spring");
                }
            }
        }
        if current_count > 0 {
            damaged_counts.push(current_count);
        }
        damaged_counts == self.damaged_groups
    }
    pub fn permutations(&self) -> Vec<SpringRow> {
        let mut v = vec![];
        let bits_count =
            2_u32.pow(self.row.iter().filter(|t| **t == Spring::Unknown).count() as u32);
        let positions: Vec<usize> = self
            .row
            .iter()
            .enumerate()
            .filter_map(|(i, t)| (*t == Spring::Unknown).then_some(i))
            .collect();
        for i in 0..bits_count {
            let mut row = self.row.clone();
            for (j, position) in positions.iter().enumerate() {
                let bit = (i >> j) & 1;
                assert!(bit == 0 || bit == 1);
                let new_spring = if bit == 0 {
                    Spring::Damaged
                } else {
                    Spring::Operational
                };
                row[*position] = new_spring;
            }
            v.push(SpringRow {
                row,
                damaged_groups: self.damaged_groups.clone(),
            })
        }
        v
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
    0
}

fn part_one(input: &str) -> usize {
    let mut sum = 0;
    let rows = parse(input);
    for row in rows {
        sum += row
            .permutations()
            .iter()
            .filter(|permutation| permutation.check_validity())
            .count();
    }
    sum
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

        // assert_eq!(part_two(INPUT_TEST), 525152);
    }
}
