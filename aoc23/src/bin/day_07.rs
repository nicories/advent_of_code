use std::{cmp::Ordering, vec};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/07_full.txt"));
const CARD_ORDERING_JOKER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
const CARD_ORDERING: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

type Hand = Vec<char>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
fn hand_type(hand: &Hand, joker: bool) -> HandType {
    let jokers = if joker {
        hand.iter().filter(|c| *c == &'J').count()
    } else {
        0
    };
    let mut duplicates: Vec<Vec<char>> = vec![];
    for c in hand {
        if joker && *c == 'J' {
            continue;
        }
        if let Some(v) = duplicates.iter_mut().find(|v| v.contains(c)) {
            v.push(*c);
        } else {
            duplicates.push(vec![*c]);
        }
    }
    duplicates.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    // 5 jokers
    if duplicates.is_empty() {
        return HandType::FiveOfAKind;
    }
    if duplicates[0].len() + jokers >= 5 {
        return HandType::FiveOfAKind;
    } else if duplicates[0].len() + jokers >= 4 {
        return HandType::FourOfAKind;
    } else if duplicates[0].len() + jokers >= 3 {
        let jokers_remaining = jokers - (3 - duplicates[0].len());
        if duplicates[1].len() + jokers_remaining >= 2 {
            return HandType::FullHouse;
        } else {
            return HandType::ThreeOfAKind;
        }
    } else if duplicates[0].len() + jokers >= 2 {
        let jokers_remaining = jokers - (2 - duplicates[0].len());
        if duplicates[1].len() + jokers_remaining >= 2 {
            return HandType::TwoPair;
        } else {
            return HandType::OnePair;
        }
    }

    HandType::HighCard
}

fn cmp_card(left: &char, right: &char, joker: bool) -> Ordering {
    let order = if joker {
        CARD_ORDERING_JOKER
    } else {
        CARD_ORDERING
    };
    let left_i = order.iter().position(|x| x == left).unwrap();
    let right_i = order.iter().position(|x| x == right).unwrap();
    left_i.cmp(&right_i)
}
fn cmp_hand(left: &Hand, right: &Hand, joker: bool) -> Ordering {
    let type_cmp = hand_type(left, joker).cmp(&hand_type(right, joker));
    if type_cmp != Ordering::Equal {
        return type_cmp;
    }
    for (i, c) in left.iter().enumerate() {
        let c_cmp = cmp_card(c, &right[i], joker);
        if c_cmp != Ordering::Equal {
            return c_cmp;
        }
    }
    Ordering::Equal
}

fn parse_hands_and_bids(input: &str, joker: bool) -> Vec<u32> {
    let mut v = vec![];
    for line in input.lines() {
        let split = line.split_once(' ').unwrap();
        v.push((split.0.chars().collect(), split.1.parse().unwrap()));
    }
    v.sort_by(|a, b| cmp_hand(&a.0, &b.0, joker));
    v.iter().map(|x| x.1).collect()
}

fn part_two(input: &str) -> u32 {
    parse_hands_and_bids(input, true)
        .iter()
        .enumerate()
        .map(|entry| (entry.0 as u32 + 1) * entry.1)
        .sum()
}

fn part_one(input: &str) -> u32 {
    parse_hands_and_bids(input, false)
        .iter()
        .enumerate()
        .map(|entry| (entry.0 as u32 + 1) * entry.1)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/07_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 6440);
        assert_eq!(part_one(INPUT), 254024898);

        assert_eq!(part_two(INPUT_TEST), 5905);
        assert_eq!(part_two(INPUT), 254115617);
    }
}
