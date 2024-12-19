const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/09_full.txt"));

#[derive(Debug, Clone, Copy)]
enum Block {
    Free,
    Used(u64),
}

#[derive(Debug)]
struct Descriptor {
    id: usize,
    length: usize,
    free_space: usize,
    free_space_occupied: usize,
    memory_index: usize,
}

fn parse(input: &str) -> (Vec<Block>, Vec<Descriptor>) {
    let layouts: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut memory = vec![];
    let mut descriptors = vec![];

    for (i, layout) in layouts.chunks(2).enumerate() {
        let desc = Descriptor {
            id: i,
            length: layout[0],
            free_space: if layout.len() > 1 { layout[1] } else { 0 },
            memory_index: memory.len(),
            free_space_occupied: 0,
        };
        for _ in 0..layout[0] {
            memory.push(Block::Used(i as u64));
        }
        if layout.len() > 1 {
            for _ in 0..layout[1] {
                memory.push(Block::Free);
            }
        }
        descriptors.push(desc);
    }
    (memory, descriptors)
}

fn part_two(input: &str) -> u64 {
    let (mut memory, mut descriptors) = parse(input);
    while let Some(desc) = descriptors.pop() {
        let head = descriptors
            .iter_mut()
            .find(|d| d.free_space >= desc.length && d.id < desc.id);
        if let Some(head) = head {
            head.free_space -= desc.length;
            for i in 0..desc.length {
                memory[desc.memory_index + i] = Block::Free;
                memory[head.memory_index + head.length + head.free_space_occupied + i] =
                    Block::Used(desc.id as u64);
            }
            head.free_space_occupied += desc.length;
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

fn part_one(input: &str) -> u64 {
    let (mut memory, _) = parse(input);
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

        assert_eq!(part_two(INPUT_TEST), 2858);
        assert_eq!(part_two(INPUT), 6511178035564);
    }
}
