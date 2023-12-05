use std::ops::Range;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_full.txt"));

#[derive(Debug)]
struct AlmanacMap {
    destination_start: Vec<usize>,
    source_start: Vec<usize>,
    range_length: Vec<usize>,
}
impl AlmanacMap {
    pub fn get_mapped(&self, v: usize) -> usize {
        for (i, source) in self.source_start.iter().enumerate() {
            let start = *source;
            let end = *source + self.range_length[i];
            if v >= start && v <= end {
                let offset = v - start;
                return self.destination_start[i] + offset;
            }
        }
        v
    }
    pub fn get_mapped_reverse(&self, v: usize) -> usize {
        for (i, destination) in self.destination_start.iter().enumerate() {
            let start = *destination;
            let end = *destination + self.range_length[i] - 1;
            if v >= start && v <= end {
                let offset = v - start;
                return self.source_start[i] + offset;
            }
        }
        v
    }
}
fn seed_to_location(seed: usize, almanacs: &Vec<AlmanacMap>) -> usize {
    let mut temp = seed;
    for al in almanacs {
        temp = al.get_mapped(temp);
    }
    temp
}
fn location_to_seed(location: usize, almanacs: &Vec<AlmanacMap>) -> usize {
    let mut temp = location;
    for al in almanacs {
        temp = al.get_mapped_reverse(temp);
    }
    temp
}

fn parse_almanac(input: &str) -> (Vec<usize>, Vec<Range<usize>>, Vec<AlmanacMap>) {
    let mut almanac = vec![];
    let mut blocks = input.split("\n\n");
    let seed_block = blocks.next().unwrap();
    let seeds: Vec<usize> = seed_block
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut seeds_ranges: Vec<Range<usize>> = vec![];
    for i in (0..seeds.len()).step_by(2) {
        let start = *seeds.get(i).unwrap();
        let length = *seeds.get(i + 1).unwrap();
        let range = Range {
            start,
            end: start + length,
        };
        seeds_ranges.push(range);
    }
    for map_block in blocks {
        let mut dest = vec![];
        let mut src = vec![];
        let mut range = vec![];

        let mut lines = map_block.lines();
        lines.next().unwrap();
        for line in lines {
            let numbers: Vec<usize> = line
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            dest.push(numbers[0]);
            src.push(numbers[1]);
            range.push(numbers[2]);
        }
        almanac.push(AlmanacMap {
            destination_start: dest,
            source_start: src,
            range_length: range,
        })
    }
    (seeds, seeds_ranges, almanac)
}

fn part_two(input: &str) -> usize {
    let (_, seed_ranges, mut almanacs) = parse_almanac(input);
    almanacs.reverse();
    for location in 0..=usize::MAX {
        let seed = location_to_seed(location, &almanacs);
        for seed_range in &seed_ranges {
            if seed_range.contains(&seed) {
                return location;
            }
        }
    }
    panic!("lul")
}

fn part_one(input: &str) -> usize {
    let (seeds, _, almanacs) = parse_almanac(input);
    seeds
        .iter()
        .map(|s| seed_to_location(*s, &almanacs))
        .min()
        .unwrap()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/05_test.txt"));

    #[test]
    fn test() {
        {
            let r = parse_almanac(INPUT_TEST);

            assert_eq!(r.0, vec![79, 14, 55, 13]);
            assert_eq!(r.2[0].destination_start, vec![50, 52]);
            assert_eq!(r.2[0].source_start, vec![98, 50]);
            assert_eq!(r.2[0].range_length, vec![2, 48]);
            assert_eq!(r.2[0].get_mapped(10), 10);
            assert_eq!(r.2[0].get_mapped(53), 55);

            assert_eq!(r.2[0].get_mapped_reverse(1), 1);
            assert_eq!(r.2[0].get_mapped_reverse(55), 53);
            assert_eq!(r.2[0].get_mapped_reverse(50), 98);

            assert_eq!(part_one(INPUT_TEST), 35);
            assert_eq!(part_one(INPUT), 600279879);
        }

        {
            assert_eq!(part_two(INPUT_TEST), 46);
            assert_eq!(part_two(INPUT), 20191102);
        }
    }
}
