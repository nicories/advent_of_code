use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/12_full.txt"));

type Graph = HashMap<String, Vec<String>>;

fn parse(input: &str) -> Graph {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        match graph.get_mut(from) {
            Some(node) => {
                node.push(to.to_string());
            }
            None => {
                graph.insert(from.to_string(), vec![to.to_string()]);
            }
        }
        // reverse
        match graph.get_mut(to) {
            Some(node) => {
                node.push(from.to_string());
            }
            None => {
                graph.insert(to.to_string(), vec![from.to_string()]);
            }
        }
    }
    graph
}

fn ways_through_caves(input: &str, allow_double_visit: bool) -> usize {
    let graph = parse(input);
    let mut queue = vec![];
    queue.push(State {
        current_node: "start".to_string(),
        visited_small: HashMap::new(),
    });
    let mut ways = 0;
    while let Some(state) = queue.pop() {
        let current = state.current_node;
        if current == "end" {
            ways += 1;
            continue;
        }
        let neighbors = graph.get(&current).unwrap();
        for neighbor in neighbors {
            let mut new_visited = state.visited_small.clone();
            // skip start
            if neighbor == "start" {
                continue;
            }

            // small cave
            if neighbor.chars().next().unwrap().is_lowercase() && neighbor != "end" {
                // already visited
                if state.visited_small.contains_key(neighbor) {
                    if !allow_double_visit {
                        continue;
                    }
                    // check if we visited a small cave once already
                    if state.visited_small.values().any(|x| *x == 2) {
                        continue;
                    }
                    new_visited.insert(neighbor.to_string(), 2);
                } else {
                    new_visited.insert(neighbor.to_string(), 1);
                }
            }
            queue.push(State {
                current_node: neighbor.to_string(),
                visited_small: new_visited
            });
        }
    }

    ways
}

fn part_two(input: &str) -> usize {
    ways_through_caves(input, true)
}

struct State {
    current_node: String,
    visited_small: HashMap<String, usize>,
}

fn part_one(input: &str) -> usize {
    ways_through_caves(input, false)
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/12_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 10);
        assert_eq!(part_one(INPUT), 3576);

        assert_eq!(part_two(INPUT_TEST), 36);
        assert_eq!(part_two(INPUT), 84271);
    }
}
