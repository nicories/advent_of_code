// the part 2 solution is simply awful but couldn't think of anything better

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/11_full.txt"));

type Position = (usize, usize);

fn pairs_from_vec(galaxies: &Vec<Position>) -> Vec<(Position, Position)> {
    let mut v = vec![];
    for (i, a) in galaxies.iter().enumerate() {
        for b in &galaxies[i + 1..] {
            v.push((a.clone(), b.clone()));
        }
    }
    v
}

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
    space: Vec<Vec<Tile>>,
}
impl Universe {
    pub fn parse(input: &str) -> Self {
        let space = input
            .lines()
            .rev()
            .map(|line| line.chars().map(|c| Tile::from_c(&c)).collect())
            .collect();
        Self { space }
    }
    pub fn galaxy_locations(&self) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for (y, row) in self.space.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if matches!(tile, Tile::Galaxy) {
                    v.push((x, y));
                }
            }
        }
        v
    }
    pub fn expand(&mut self) {
        // rows
        let mut row_indices: Vec<usize> = vec![];
        for (row_index, row) in self.space.iter().enumerate() {
            if !row.contains(&Tile::Galaxy) {
                row_indices.push(row_index);
            }
        }
        for (i, index) in row_indices.iter().enumerate() {
            let new_row = self.space[i + index].to_vec();
            self.space.insert(i + index, new_row);
        }
        // columns
        let mut column_indices: Vec<usize> = vec![];
        for column_index in 0..self.space[0].len() {
            let mut no_galaxy = true;
            for row_index in 0..self.space.len() {
                if matches!(self.space[row_index][column_index], Tile::Galaxy) {
                    no_galaxy = false;
                    break;
                }
            }
            if no_galaxy {
                column_indices.push(column_index);
            }
        }
        for (i, index) in column_indices.iter().enumerate() {
            for row in &mut self.space {
                row.insert(index + i, Tile::Empty);
            }
        }
    }
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let mut universe = Universe::parse(input);
    universe.expand();
    let galaxies = universe.galaxy_locations();
    let pairs = pairs_from_vec(&galaxies);
    pairs
        .iter()
        .map(|pair| pair.0 .0.abs_diff(pair.1 .0) + pair.0 .1.abs_diff(pair.1 .1))
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/11_test.txt"));

    #[test]
    fn test() {
        let mut universe = Universe::parse(INPUT_TEST);
        assert_eq!(universe.galaxy_locations().len(), 9);
        assert_eq!(universe.space.len(), 10);
        for row in &universe.space {
            assert_eq!(row.len(), 10);
        }

        universe.expand();
        assert_eq!(universe.space.len(), 12);
        for row in &universe.space {
            assert_eq!(row.len(), 13);
        }
        let galaxies = universe.galaxy_locations();
        assert_eq!(galaxies.len(), 9);
        let pairs = pairs_from_vec(&galaxies);

        assert_eq!(pairs.len(), 36);
        assert_eq!(part_one(INPUT_TEST), 374);
        // assert_eq!(part_one(INPUT), 6768);
        // assert_eq!(part_two(INPUT), 351);
    }
}
