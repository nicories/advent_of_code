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
fn hand_type_joker(hand: &Hand) -> HandType {
    let jokers = hand.iter().filter(|c| *c == &'J').count();
    let mut vecs: Vec<Vec<char>> = vec![];
    for c in hand {
        if *c == 'J' {
            continue;
        }
        let mut new = true;
        for v in &mut vecs {
            if v.contains(c) {
                new = false;
                v.push(*c);
            }
        }
        if new {
            vecs.push(vec![*c]);
        }
    }
    vecs.sort_by(|a, b| a.len().cmp(&b.len()));
    vecs.reverse();
    // 5 jokers
    if vecs.is_empty() {
        return HandType::FiveOfAKind;
    }
    if vecs[0].len() + jokers >= 5 {
        return HandType::FiveOfAKind;
    } else if vecs[0].len() + jokers >= 4 {
        return HandType::FourOfAKind;
    } else if vecs[0].len() + jokers >= 3 {
        let jokers_remaining = jokers - (3 - vecs[0].len());
        if vecs[1].len() + jokers_remaining >= 2 {
            return HandType::FullHouse;
        } else {
            return HandType::ThreeOfAKind;
        }
    } else if vecs[0].len() + jokers >= 2 {
        let jokers_remaining = jokers - (2 - vecs[0].len());
        if vecs[1].len() + jokers_remaining >= 2 {
            return HandType::TwoPair;
        } else {
            return HandType::OnePair;
        }
    }

    HandType::HighCard
}
fn hand_type(hand: &Hand) -> HandType {
    let mut vecs: Vec<Vec<char>> = vec![];
    for c in hand {
        let mut new = true;
        for v in &mut vecs {
            if v.contains(c) {
                new = false;
                v.push(*c);
            }
        }
        if new {
            vecs.push(vec![*c]);
        }
    }
    vecs.sort_by(|a, b| a.len().cmp(&b.len()));
    vecs.reverse();
    if vecs[0].len() == 5 {
        return HandType::FiveOfAKind;
    } else if vecs[0].len() == 4 {
        return HandType::FourOfAKind;
    } else if vecs[0].len() == 3 {
        if vecs[1].len() == 2 {
            return HandType::FullHouse;
        } else {
            return HandType::ThreeOfAKind;
        }
    } else if vecs[0].len() == 2 {
        if vecs[1].len() == 2 {
            return HandType::TwoPair;
        } else {
            return HandType::OnePair;
        }
    }

    HandType::HighCard
}
fn cmp_card(left: &char, right: &char, joker: bool) -> Ordering {
    let mut v = if joker {
        vec![
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ]
    } else {
        vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ]
    };
    v.reverse();
    let left_i = v.iter().position(|x| x == left).unwrap();
    let right_i = v.iter().position(|x| x == right).unwrap();
    left_i.cmp(&right_i)
}
fn cmp_hand(left: &Hand, right: &Hand, joker: bool) -> Ordering {
    let type_cmp = if joker {
        hand_type_joker(left).cmp(&hand_type_joker(right))
    } else {
        hand_type(left).cmp(&hand_type(right))
    };
    if type_cmp != Ordering::Equal {
        return type_cmp;
    }
    for (i, c) in left.iter().enumerate() {
        let c_cmp = cmp_card(c, &right[i], joker);
        dbg!("{} {} {}", c, right[i], &c_cmp);
        if c_cmp != Ordering::Equal {
            return c_cmp;
        }
    }
    Ordering::Equal
}

fn parse_hands_and_bids(input: &str) -> Vec<(Hand, u32)> {
    let mut v = vec![];
    for line in input.lines() {
        let split = line.split_once(' ').unwrap();
        v.push((split.0.chars().collect(), split.1.parse().unwrap()));
    }
    v
}

fn part_two(input: &str) -> u32 {
    let mut sum = 0;
    let mut hands_and_bids = parse_hands_and_bids(input);
    hands_and_bids.sort_by(|a, b| cmp_hand(&a.0, &b.0, true));
    dbg!(&hands_and_bids);
    for (i, entry) in hands_and_bids.iter().enumerate() {
        sum += (i as u32 + 1) * entry.1;
    }
    sum
}

fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    let mut hands_and_bids = parse_hands_and_bids(input);
    hands_and_bids.sort_by(|a, b| cmp_hand(&a.0, &b.0, false));
    dbg!(&hands_and_bids);
    for (i, entry) in hands_and_bids.iter().enumerate() {
        sum += (i as u32 + 1) * entry.1;
    }
    sum
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
        let mut x = parse_hands_and_bids(INPUT_TEST);
        assert_eq!(x[0], (vec!['3', '2', 'T', '3', 'K'], 765));
        assert_eq!(hand_type(&x[0].0), HandType::OnePair);
        assert_eq!(hand_type(&x[1].0), HandType::ThreeOfAKind);
        assert_eq!(hand_type(&x[2].0), HandType::TwoPair);
        x.sort_by(|a, b| cmp_hand(&a.0, &b.0, false));
        assert_eq!(x[0].1, 765);
        assert_eq!(x[1].1, 220);
        assert_eq!(x[2].1, 28);
        assert_eq!(x[3].1, 684);
        assert_eq!(x[4].1, 483);
        assert_eq!(part_one(INPUT_TEST), 6440);
        assert_eq!(part_one(INPUT), 254024898);

        assert_eq!(
            hand_type_joker(&vec!['Q', 'Q', 'Q', 'J', 'Q']),
            HandType::FiveOfAKind
        );
        assert_eq!(
            hand_type_joker(&vec!['J', 'Q', 'Q', 'J', 'Q']),
            HandType::FiveOfAKind
        );
        assert_eq!(
            hand_type_joker(&vec!['J', 'Q', 'Q', 'J', 'Q']),
            HandType::FiveOfAKind
        );
        assert_eq!(
            hand_type_joker(&vec!['J', 'J', 'Q', 'J', 'Q']),
            HandType::FiveOfAKind
        );
        assert_eq!(
            hand_type_joker(&vec!['J', 'J', 'J', 'J', 'Q']),
            HandType::FiveOfAKind
        );
        assert_eq!(
            hand_type_joker(&vec!['J', 'J', 'J', 'J', 'J']),
            HandType::FiveOfAKind
        );
        assert_eq!(
            hand_type_joker(&vec!['J', 'A', 'A', 'Q', 'Q']),
            HandType::FullHouse
        );
        assert_eq!(
            hand_type_joker(&vec!['J', 'A', 'A', '2', 'Q']),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            hand_type_joker(&vec!['A', '2', '3', '4', '5']),
            HandType::HighCard
        );
        assert_eq!(
            hand_type_joker(&vec!['A', '2', '3', '4', 'J']),
            HandType::OnePair
        );
        assert_eq!(part_two(INPUT_TEST), 5905);
        assert_eq!(part_two(INPUT), 254115617);
    }
}
