const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_full.txt"));

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_highest_batteries(pack: &Vec<u32>, num_of_batteries: u32) -> u64 {
    let mut highest_indices: Vec<usize> = vec![];
    for i in 0..num_of_batteries as usize {
        let mut highest_index = if i == 0 {
            0
        } else {
            highest_indices[i - 1] + 1
        };
        let start = highest_index + 1;
        let stop = pack.len() - num_of_batteries as usize + i + 1;
        for j in start..stop {
            if pack[j] > pack[highest_index] {
                highest_index = j;
            }
        }
        highest_indices.push(highest_index);
    }
    highest_indices
        .iter()
        .enumerate()
        .map(|(i, pack_index)| {
            pack[*pack_index] as u64 * 10_u64.pow(num_of_batteries - 1 - i as u32)
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|pack| find_highest_batteries(pack, 12))
        .sum()
}

fn part_one(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|pack| find_highest_batteries(pack, 2))
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/03_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 357);
        assert_eq!(part_one(INPUT), 17166);

        assert_eq!(part_two(INPUT_TEST), 3121910778619);
        assert_eq!(part_two(INPUT), 169077317650774);
    }
}
