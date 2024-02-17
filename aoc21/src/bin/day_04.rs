const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/04_full.txt"));

type BingoBoard = Vec<Vec<u32>>;

fn parse(input: &str) -> (Vec<BingoBoard>, Vec<u32>) {
    let mut split = input.split("\n\n");
    let numbers = split
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let boards = split
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|c| {
                            dbg!(&c);
                            c.parse::<u32>().unwrap()
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    (boards, numbers)
}
fn check_bingo(board: &BingoBoard, numbers: &[u32]) -> bool {
    // horizontal
    for line in board {
        if line.iter().all(|n| numbers.contains(n)) {
            return true;
        }
    }
    // vertical
    for i in 0..board[0].len() {
        let vertical_column: Vec<_> = board.iter().map(|inner_vec| inner_vec[i]).collect();
        if vertical_column.iter().all(|n| numbers.contains(n)) {
            return true;
        }
    }
    false
}
fn part_two(input: &str) -> u32 {
    0
}

fn part_one(input: &str) -> u32 {
    let (boards, numbers) = parse(input);
    let mut drawn_numbers = vec![];
    for number in numbers {
        drawn_numbers.push(number);
        for board in &boards {
            if check_bingo(board, &drawn_numbers) {
                return number
                    * board
                        .iter()
                        .map(|line| {
                            line.iter()
                                .filter(|x| !drawn_numbers.contains(x))
                                .sum::<u32>()
                        })
                        .sum::<u32>();
            }
        }
    }
    panic!("at the disco");
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
        assert_eq!(part_one(INPUT_TEST), 4512);
        assert_eq!(part_one(INPUT), 8442);

        // assert_eq!(part_two(INPUT_TEST), 230);
        // assert_eq!(part_two(INPUT), 1007985);
    }
}
