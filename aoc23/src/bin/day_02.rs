const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/02_full.txt"));

#[derive(PartialEq, Debug)]
struct Rgb {
    red: u32,
    green: u32,
    blue: u32,
}
struct Game {
    id: u32,
    sets: Vec<Rgb>,
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut v = vec![];
    for line in input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        let game_id = str::parse::<u32>(split.first().unwrap().split(' ').last().unwrap()).unwrap();
        let sets_split = split.last().unwrap().split(';');
        let mut sets = vec![];
        for set in sets_split {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let rgb_split = set.trim().split(',');
            for color in rgb_split {
                let color_split: Vec<&str> = color.trim().split(' ').collect();
                let count = str::parse::<u32>(color_split.first().unwrap()).unwrap();
                match *color_split.last().unwrap() {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => panic!(),
                }
            }
            sets.push(Rgb { red, green, blue })
        }
        v.push(Game { id: game_id, sets })
    }
    v
}

fn part_two(input: &str) -> u32 {
    let games = parse_games(input);
    let mut sum = 0;
    for game in games {
        let red = game.sets.iter().max_by_key(|x| x.red).unwrap().red;
        let blue = game.sets.iter().max_by_key(|x| x.blue).unwrap().blue;
        let green = game.sets.iter().max_by_key(|x| x.green).unwrap().green;
        sum += red * blue * green;
    }
    sum
}

fn part_one(input: &str) -> u32 {
    let games = parse_games(input);
    let possible_games = games.iter().filter(|game| {
        game.sets
            .iter()
            .filter(|set| set.red > 12 || set.green > 13 || set.blue > 14)
            .count()
            == 0
    });
    possible_games.map(|game| game.id).sum()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/02_test.txt"));

    #[test]
    fn test() {
        let games = parse_games(INPUT_TEST);
        assert_eq!(games[0].id, 1);
        assert_eq!(
            games[0].sets,
            vec![
                Rgb {
                    blue: 3,
                    red: 4,
                    green: 0
                },
                Rgb {
                    blue: 6,
                    red: 1,
                    green: 2
                },
                Rgb {
                    blue: 0,
                    red: 0,
                    green: 2
                },
            ]
        );
        assert_eq!(part_one(INPUT_TEST), 8);
        assert_eq!(part_one(INPUT), 2505);

        assert_eq!(part_two(INPUT_TEST), 2286);
        assert_eq!(part_two(INPUT), 70265);
    }
}
