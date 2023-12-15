const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/15_full.txt"));

type Box = Vec<(String, usize)>;

fn hash_algorithm(input: &str) -> u32 {
    let mut value = 0;
    for c in input.chars() {
        // ignore newline
        if c == '\n' {
            continue;
        }
        let ascii_code = c as u32;
        value += ascii_code;
        value *= 17;
        value %= 256;
    }
    value
}

fn part_two(input: &str) -> usize {
    let mut boxes: Vec<Box> = vec![];
    for _ in 0..256 {
        boxes.push(vec![]);
    }
    for operation in input.split(',') {
        if operation.contains('=') {
            let (label, focal_length) = operation.split_once('=').unwrap();
            let focal_length: usize = focal_length.parse().unwrap();
            let box_id = hash_algorithm(label);
            let s_box = &mut boxes[box_id as usize];
            if let Some(lens) = s_box.iter_mut().find(|lens| lens.0 == label) {
                lens.1 = focal_length;
            } else {
                s_box.push((label.to_string(), focal_length));
            }
        } else {
            let (label, _) = operation.split_once('-').unwrap();
            for light_box in &mut boxes {
                if let Some(index) = light_box.iter().position(|x| x.0 == label) {
                    light_box.remove(index);
                }
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(box_index, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_index, lens)| (lens_index + 1) * (box_index + 1) * lens.1)
                .sum::<usize>()
        })
        .sum()
}

fn part_one(input: &str) -> u32 {
    input.split(',').map(|x| hash_algorithm(x)).sum()
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/15_test.txt"));

    #[test]
    fn test() {
        assert_eq!(hash_algorithm("HASH"), 52);
        assert_eq!(part_one(INPUT_TEST), 1320);
        assert_eq!(part_one(INPUT), 514281);

        assert_eq!(part_two(INPUT_TEST), 145);
        assert_eq!(part_two(INPUT), 244199);
    }
}
