use std::ops::RangeInclusive;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/24_full.txt"));

type Position = (i64, i64, i64);

#[derive(Debug, Clone, PartialEq)]
struct HailStone {
    position: Position,
    velocity: (i64, i64, i64),
    x: LinearFunction,
    y: LinearFunction,
    z: LinearFunction,
}

fn parse_hailstones(input: &str) -> Vec<HailStone> {
    input
        .lines()
        .map(|line| {
            let (position_str, velo_str) = line.split_once('@').unwrap();
            let positions: Vec<i64> = position_str
                .split(',')
                .map(|s| s.trim())
                .map(|s| s.parse().unwrap())
                .collect();
            let velos: Vec<i64> = velo_str
                .split(',')
                .map(|s| s.trim())
                .map(|s| s.parse().unwrap())
                .collect();
            HailStone {
                position: (positions[0], positions[1], positions[2]),
                velocity: (velos[0], velos[1], velos[2]),
                x: LinearFunction {
                    velocity: velos[0],
                    start: positions[0],
                },
                y: LinearFunction {
                    velocity: velos[1],
                    start: positions[1],
                },
                z: LinearFunction {
                    velocity: velos[2],
                    start: positions[2],
                },
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct LinearFunction {
    velocity: i64,
    start: i64,
}

fn get_possible_velocities(a: &LinearFunction, b: &LinearFunction) -> Vec<i64> {
    let mut v = vec![];
    let diff = if a.start > b.start {
        a.start - b.start
    } else {
        b.start - a.start
    };
    for velo in -3000..3000 {
        if velo != a.velocity && diff % (velo - a.velocity) == 0 {
            v.push(velo);
        }
    }

    v
}

fn intersection_xy(a: &HailStone, b: &HailStone) -> Option<(f64, f64)> {
    if a.velocity.1 * b.velocity.0 == b.velocity.1 * a.velocity.0 {
        return None;
    }
    let a_m = a.velocity.1 as f64 / a.velocity.0 as f64;
    let a_b = a.position.1 as f64 - a.position.0 as f64 * a_m;

    let b_m = b.velocity.1 as f64 / b.velocity.0 as f64;
    let b_b = b.position.1 as f64 - b.position.0 as f64 * b_m;

    let x = (b_b - a_b) as f64 / (a_m - b_m) as f64;
    let y = a_m * x + a_b;
    if a.velocity.0 > 0 {
        if x < a.position.0 as f64 {
            return None;
        }
    } else {
        if x > a.position.0 as f64 {
            return None;
        }
    }

    if b.velocity.0 > 0 {
        if x < b.position.0 as f64 {
            return None;
        }
    } else {
        if x > b.position.0 as f64 {
            return None;
        }
    }

    Some((x, y))
}

fn future_intersections_xy(input: &str, test_area: RangeInclusive<f64>) -> Vec<(f64, f64)> {
    let hailstones = parse_hailstones(input);
    let mut v = vec![];
    for (i, a) in hailstones.iter().enumerate() {
        for b in &hailstones[i + 1..] {
            if let Some(intersection) = intersection_xy(a, b) {
                if test_area.contains(&intersection.0) && test_area.contains(&intersection.1) {
                    v.push(intersection);
                }
            }
        }
    }
    v
}
fn find_parallel(functions: &Vec<LinearFunction>) -> Vec<(LinearFunction, LinearFunction)> {
    let mut v = vec![];
    for (i, a) in functions.iter().enumerate() {
        for b in &functions[i + 1..] {
            if a.velocity == b.velocity {
                v.push((a.clone(), b.clone()));
            }
        }
    }
    assert!(!v.is_empty());
    v
}

fn find_velocities(functions: &Vec<LinearFunction>) -> Vec<i64> {
    let mut parallel_pairs = find_parallel(functions);
    dbg!(&parallel_pairs);
    let first_pair = parallel_pairs.pop().unwrap();
    let mut candidates = get_possible_velocities(&first_pair.0, &first_pair.1);
    for (a, b) in parallel_pairs {
        if candidates.len() == 1 {
            break;
        }
        let velos = get_possible_velocities(&a, &b);
        candidates.retain(|c| velos.contains(c));
    }
    dbg!(&candidates);
    candidates
}

fn part_two(input: &str) -> i64 {
    let hailstones = parse_hailstones(input);
    let x_functions = hailstones.iter().map(|h| h.x).collect();
    let y_functions = hailstones.iter().map(|h| h.y).collect();
    let z_functions = hailstones.iter().map(|h| h.z).collect();
    let x_velos = find_velocities(&x_functions);
    let y_velos = find_velocities(&y_functions);
    let z_velos = find_velocities(&z_functions);
    dbg!(&x_velos, &y_velos, &z_velos);

    let x_velocity = x_velos[0];
    let y_velocity = y_velos[0];
    let z_velocity = z_velos[0];

    let a = &hailstones[0];

    let b = &hailstones[1];

    let ma = (a.y.velocity - y_velocity) as f64 / (a.x.velocity - x_velocity) as f64;
    let mb = (b.y.velocity - y_velocity) as f64 / (b.x.velocity - x_velocity) as f64;

    let ca = a.y.start as f64 - (ma * a.x.start as f64);
    let cb = b.y.start as f64 - (mb * b.x.start as f64);

    let x_pos = ((cb - ca) / (ma - mb)).round() as i64;

    let y_pos = (ma * x_pos as f64 + ca).round() as i64;

    let time = (x_pos - a.x.start) / (a.x.velocity - x_velocity);

    let z_pos = a.z.start + (a.z.velocity - z_velocity) * time;

    x_pos + y_pos + z_pos
}

fn part_one(input: &str) -> usize {
    future_intersections_xy(
        input,
        RangeInclusive::new(200000000000000.0, 400000000000000.0),
    )
    .len()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/24_test.txt"));

    #[test]
    fn test() {
        let hailstones = parse_hailstones(INPUT_TEST);
        assert!(intersection_xy(&hailstones[0], &hailstones[1]).is_some());
        assert!(intersection_xy(&hailstones[0], &hailstones[2]).is_some());
        assert!(intersection_xy(&hailstones[0], &hailstones[3]).is_some());
        assert!(intersection_xy(&hailstones[0], &hailstones[4]).is_none());
        let intersections = future_intersections_xy(INPUT_TEST, RangeInclusive::new(7.0, 27.0));
        assert_eq!(intersections.len(), 2);
        assert_eq!(part_one(INPUT), 14672);

        // assert_eq!(part_two(INPUT_TEST), 47);
        assert_eq!(part_two(INPUT), 646810057104753);
    }
}
