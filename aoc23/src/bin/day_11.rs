// the part 2 solution is simply awful but couldn't think of anything better

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/11_full.txt"));

type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Galaxy,
    Empty,
}
impl Tile {
    pub fn from_c(c: &char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("unexpected char"),
        }
    }
}
#[derive(Debug)]
struct Universe {
    original_space: Vec<Vec<Tile>>,
    galaxy_locations: Vec<Position>,
}
impl Universe {
    pub fn parse(input: &str) -> Self {
        let space: Vec<Vec<Tile>> = input
            .lines()
            .rev()
            .map(|line| line.chars().map(|c| Tile::from_c(&c)).collect())
            .collect();
        let mut galaxy_locations = vec![];
        for (y, row) in space.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if matches!(tile, Tile::Galaxy) {
                    galaxy_locations.push((x, y));
                }
            }
        }
        Self {
            original_space: space,
            galaxy_locations,
        }
    }
    pub fn expand(&mut self, factor: usize) {
        // rows
        let row_indices: Vec<usize> = self
            .original_space
            .iter()
            .enumerate()
            .filter_map(|(i, row)| (!row.contains(&Tile::Galaxy)).then_some(i))
            .collect();
        // columns
        let mut column_indices: Vec<usize> = vec![];
        for column_index in 0..self.original_space[0].len() {
            let mut no_galaxy = true;
            for row_index in 0..self.original_space.len() {
                if matches!(self.original_space[row_index][column_index], Tile::Galaxy) {
                    no_galaxy = false;
                    break;
                }
            }
            if no_galaxy {
                column_indices.push(column_index);
            }
        }
        for galaxy in &mut self.galaxy_locations {
            let x_offset = column_indices
                .iter()
                .filter(|index| **index < galaxy.0)
                .count();
            let y_offset = row_indices
                .iter()
                .filter(|index| **index < galaxy.1)
                .count();
            galaxy.0 += x_offset * (factor - 1);
            galaxy.1 += y_offset * (factor - 1);
        }
    }
    fn galaxy_pairs(&self) -> Vec<(Position, Position)> {
        let mut pairs = vec![];
        for (i, a) in self.galaxy_locations.iter().enumerate() {
            for b in &self.galaxy_locations[i + 1..] {
                pairs.push((*a, *b));
            }
        }
        pairs
    }
}

fn galaxy_paths(input: &str, factor: usize) -> usize {
    let mut universe = Universe::parse(input);
    universe.expand(factor);
    universe
        .galaxy_pairs()
        .iter()
        .map(|pair| pair.0 .0.abs_diff(pair.1 .0) + pair.0 .1.abs_diff(pair.1 .1))
        .sum()
}

fn part_two(input: &str) -> usize {
    galaxy_paths(input, 1_000_000)
}

fn part_one(input: &str) -> usize {
    galaxy_paths(input, 2)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/11_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 374);
        assert_eq!(part_one(INPUT), 9647174);
        assert_eq!(galaxy_paths(INPUT_TEST, 10), 1030);
        assert_eq!(galaxy_paths(INPUT_TEST, 100), 8410);
        assert_eq!(part_two(INPUT), 377318892554);
    }
}
