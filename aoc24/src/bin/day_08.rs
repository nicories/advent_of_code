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

fn count_unique_antinodes(input: &str, limit_positions: bool) -> usize {
    let (points_vec, x_limit, y_limit) = parse(input);
    let mut antinodes: Vec<Point> = points_vec
        .values()
        .flat_map(|points| {
            let mut v = vec![];
            for (i, a) in points.iter().enumerate() {
                for b in points.iter().skip(i + 1) {
                    let x_diff = a.x - b.x;
                    let y_diff = a.y - b.y;
                    if limit_positions {
                        v.push(Point {
                            x: a.x + x_diff,
                            y: a.y + y_diff,
                        });
                        v.push(Point {
                            x: b.x - x_diff,
                            y: b.y - y_diff,
                        });
                    } else {
                        let mut x = a.x;
                        let mut y = a.y;
                        while (0..x_limit).contains(&x) && (0..y_limit).contains(&y) {
                            v.push(Point { x, y });
                            x -= x_diff;
                            y -= y_diff;
                        }
                        x = b.x;
                        y = b.y;
                        while (0..x_limit).contains(&x) && (0..y_limit).contains(&y) {
                            v.push(Point { x, y });
                            x += x_diff;
                            y += y_diff;
                        }
                    }
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

fn part_two(input: &str) -> usize {
    count_unique_antinodes(input, false)
}

fn part_one(input: &str) -> usize {
    count_unique_antinodes(input, true)
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

        assert_eq!(part_two(INPUT_TEST), 34);
        assert_eq!(part_two(INPUT), 1308);
    }
}
