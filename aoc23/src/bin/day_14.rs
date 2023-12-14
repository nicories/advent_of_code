use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/14_full.txt"));

struct Dish {
    grid: Vec<Vec<char>>,
}
impl Dish {
    pub fn parse(input: &str) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();
        Dish { grid }
    }
    pub fn rotate(&mut self) {
        let mut new_grid = vec![];
        for x in 0..self.grid[0].len() {
            let mut new_line = vec![];
            for y in (0..self.grid.len()).rev() {
                new_line.push(self.grid[y][x]);
            }
            new_grid.push(new_line);
        }
        self.grid = new_grid;
    }
    pub fn tilt_north(&mut self) {
        for y in 1..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == 'O' {
                    let mut offset = 0;
                    self.grid[y][x] = '.';
                    while y - offset > 0 && self.grid[y - offset - 1][x] == '.' {
                        offset += 1;
                    }
                    self.grid[y - offset][x] = 'O';
                }
            }
        }
    }
    pub fn calculate_load(&self) -> usize {
        self.grid
            .iter()
            .rev()
            .enumerate()
            .map(|line| (line.0 + 1) * line.1.iter().filter(|tile| **tile == 'O').count())
            .sum()
    }
}

fn part_two(input: &str) -> usize {
    let mut dish = Dish::parse(input);
    let mut map: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let repeat = 1000000000;
    for i in 0..repeat {
        for _ in 0..4 {
            dish.tilt_north();
            dish.rotate();
        }
        if let Some(last_i) = map.get(&dish.grid) {
            let loop_size = i - last_i;
            let remaining_loops = (repeat - i) % loop_size - 1;
            for _ in 0..remaining_loops {
                for _ in 0..4 {
                    dish.tilt_north();
                    dish.rotate();
                }
            }
            return dish.calculate_load();
        }
        map.insert(dish.grid.to_vec(), i);
    }
    panic!("at the disco");
}

fn part_one(input: &str) -> usize {
    let mut dish = Dish::parse(input);
    dish.tilt_north();
    dish.calculate_load()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/14_test.txt"));

    #[test]
    fn test() {
        let mut dish = Dish::parse(INPUT_TEST);
        assert_eq!(
            String::from(dish.grid[0].iter().collect::<String>()),
            "O....#...."
        );
        dish.tilt_north();
        assert_eq!(
            String::from(dish.grid[0].iter().collect::<String>()),
            "OOOO.#.O.."
        );

        assert_eq!(part_one(INPUT_TEST), 136);
        assert_eq!(part_one(INPUT), 109661);

        assert_eq!(part_two(INPUT_TEST), 64);
        assert_eq!(part_two(INPUT), 90176);
    }
}
