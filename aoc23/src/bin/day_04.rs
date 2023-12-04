const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/04_full.txt"));

type Card = (Vec<u32>, Vec<u32>);
fn parse_numbers(input: &str) -> Vec<Card> {
    let mut v = vec![];
    for line in input.lines() {
        let numbers_split = line.split_once(':').unwrap().1.split_once('|').unwrap();

        let winning_numbers: Vec<u32> = numbers_split
            .0
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let my_numbers: Vec<u32> = numbers_split
            .1
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        v.push((winning_numbers, my_numbers));
    }
    v
}

fn part_two(input: &str) -> u32 {
    let cards = parse_numbers(input);
    let mut cards_with_count: Vec<(u32, &Card)> = cards.iter().map(|x| (1, x)).collect();
    for i in 0..cards_with_count.len() {
        let card_count = cards_with_count.get(i).unwrap().0;
        let wins = &cards_with_count.get(i).unwrap().1 .0;
        let my = &cards_with_count.get(i).unwrap().1 .1;
        let winning_count = my.iter().filter(|d| wins.contains(d)).count() as u32;
        for j in 1..=winning_count {
            cards_with_count.get_mut(i + j as usize).unwrap().0 += card_count;
        }
    }

    cards_with_count.iter().map(|x| x.0).sum()
}

fn part_one(input: &str) -> u32 {
    parse_numbers(input)
        .iter()
        .map(|(win, my)| my.iter().filter(|d| win.contains(d)).count())
        .filter(|&win_count| win_count > 0)
        .map(|winning_numbers| 2_u32.pow(winning_numbers as u32 - 1))
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/04_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 13);
        assert_eq!(part_one(INPUT), 20667);

        assert_eq!(part_two(INPUT_TEST), 30);
        assert_eq!(part_two(INPUT), 5833065);
    }
}
