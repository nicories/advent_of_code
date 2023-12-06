const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_full.txt"));

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_times_and_distances(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut lines = input.lines().map(|l| l.split_once(':').unwrap().1);
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (times, distances)
}

fn digit_concat(v: Vec<u64>) -> u64 {
    let number_str: String = v.iter().map(|&digit| digit.to_string()).collect();
    number_str.parse().unwrap_or_default()
}

fn ways_to_win_race(race: &Race) -> usize {
    let mut ways = 0;
    for time in 0..race.time {
        let speed = time;
        let remaining_time = race.time - time;
        if speed * remaining_time > race.distance {
            ways += 1;
        }
    }
    ways
}

fn part_two(input: &str) -> usize {
    let (times, distances) = parse_times_and_distances(input);
    let time = digit_concat(times);
    let distance = digit_concat(distances);
    let race = Race { time, distance };
    ways_to_win_race(&race)
}

fn part_one(input: &str) -> usize {
    let mut races = vec![];
    let (times, distances) = parse_times_and_distances(input);
    for (i, time) in times.iter().enumerate() {
        races.push(Race {
            time: *time,
            distance: distances[i],
        });
    }
    races.iter().map(|race| ways_to_win_race(race)).product()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_test.txt"));
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/06_test2.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 288);
        assert_eq!(part_one(INPUT_TEST2), 71503);
        assert_eq!(part_one(INPUT), 1083852);

        assert_eq!(part_two(INPUT_TEST), 71503);
        assert_eq!(part_two(INPUT), 23501589);
    }
}
