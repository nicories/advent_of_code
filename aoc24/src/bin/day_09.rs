const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_full.txt"));

#[derive(Debug, Clone, Copy)]
enum Block {
    Free,
    Used(u64),
}

fn parse(input: &str) -> Vec<Block> {
    let layouts: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut memory = vec![];

    for (i, layout) in layouts.chunks(2).enumerate() {
        for _ in 0..layout[0] {
            memory.push(Block::Used(i as u64));
        }
        if layout.len() > 1 {
            for _ in 0..layout[1] {
                memory.push(Block::Free);
            }
        }
    }
    memory
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> u64 {
    let mut memory = parse(input);
    let mut inner = 0;
    let mut outer = memory.len() - 1;
    while outer > inner {
        match (memory[inner], memory[outer]) {
            (Block::Free, Block::Free) => outer -= 1,
            (Block::Free, Block::Used(outer_value)) => {
                memory[inner] = Block::Used(outer_value);
                memory[outer] = Block::Free;
            }
            (Block::Used(_), _) => inner += 1,
        }
    }

    memory
        .into_iter()
        .enumerate()
        .map(|(index, block)| match block {
            Block::Free => 0,
            Block::Used(id) => index as u64 * id,
        })
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 1928);
        assert_eq!(part_one(INPUT), 6471961544878);

        // assert_eq!(part_two(INPUT_TEST), 34);
        // assert_eq!(part_two(INPUT), 1308);
    }
}
