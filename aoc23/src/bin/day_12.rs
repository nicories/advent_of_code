use std::collections::HashMap;

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
    springs: Vec<Spring>,
    groups: Vec<usize>,
}
impl SpringRow {
    pub fn count_arrangements(
        &self,
        spring_index: usize,
        group_index: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(res) = cache.get(&(spring_index, group_index)) {
            return *res;
        }
        // skip all operational springs
        if spring_index < self.springs.len() && self.springs[spring_index] == Spring::Operational {
            return self.count_arrangements(spring_index + 1, group_index, cache);
        }
        let Some(group) = self.groups.get(group_index).copied() else {
            return 0;
        };

        if spring_index + group > self.springs.len() {
            return 0;
        }

        let mut result = 0;

        let target_springs = &self.springs[spring_index..spring_index + group];
        let (count_damaged, count_unknown) = target_springs.iter().fold((0, 0), |acc, x| match x {
            Spring::Damaged => (acc.0 + 1, acc.1),
            Spring::Unknown => (acc.0, acc.1 + 1),
            _ => acc,
        });
        // this checks if the current group is valid assuming that all unknowns can be turned into damaged springs
        if count_damaged <= group
            && count_damaged + count_unknown >= group
            && !(spring_index + group < self.springs.len()
                && self.springs[spring_index + group] == Spring::Damaged)
        {
            // if all groups are processed the row is valid if
            // there are no springs or no more damaged springs left
            if group_index == self.groups.len() - 1
                && (spring_index + group + 1 >= self.springs.len()
                    || !self.springs[spring_index + group + 1..].contains(&Spring::Damaged))
            {
                result += 1;
            } else {
                // if we still have groups to process, defer to the result of the next group
                result += self.count_arrangements(spring_index + group + 1, group_index + 1, cache);
            }
        }
        // also check the permutation where the first unknown is operational
        if self.springs[spring_index] == Spring::Unknown {
            result += self.count_arrangements(spring_index + 1, group_index, cache);
        }
        cache.insert((spring_index, group_index), result);
        result
    }

    pub fn unfold(&self) -> Self {
        let mut new_springs = self.springs.clone();
        let mut new_groups = self.groups.clone();
        for _ in 0..4 {
            new_springs.push(Spring::Unknown);
            new_springs.extend(self.springs.clone());
            new_groups.extend(self.groups.clone());
        }
        Self {
            springs: new_springs,
            groups: new_groups,
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
            springs: row,
            groups: damaged_groups,
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

        assert_eq!(part_two(INPUT_TEST), 525152);
        assert_eq!(part_two(INPUT), 6720660274964);
    }
}
