use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_full.txt"));

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> (HashMap<char, Vec<Point>>, i32, i32) {
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };
                if let Some(v) = map.get_mut(&c) {
                    v.push(p);
                } else {
                    map.insert(c, vec![p]);
                }
            }
        });
    });
    (
        map,
        input.lines().next().unwrap().chars().count() as i32,
        input.lines().count() as i32,
    )
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let (points_vec, x_limit, y_limit) = parse(input);
    let mut antinodes: Vec<Point> = points_vec
        .values()
        .flat_map(|points| {
            let mut v = vec![];
            for i in 0..points.len() - 1 {
                let a = &points[i];
                for j in i + 1..points.len() {
                    let b = &points[j];
                    v.push(Point {
                        x: 2 * a.x - b.x,
                        y: 2 * a.y - b.y,
                    });
                    v.push(Point {
                        x: 2 * b.x - a.x,
                        y: 2 * b.y - a.y,
                    });
                }
            }
            v
        })
        .collect();
    antinodes.retain(|p| p.x >= 0 && p.x < x_limit && p.y >= 0 && p.y < y_limit);
    antinodes.sort();
    antinodes.dedup();
    antinodes.len()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/08_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 14);
        assert_eq!(part_one(INPUT), 409);

        // assert_eq!(part_two(INPUT_TEST), 11387);
        // assert_eq!(part_two(INPUT), 92612386119138);
    }
}
