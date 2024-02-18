use std::collections::{HashMap, HashSet};

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

fn part_two(input: &str) -> usize {
    0
}

struct State {
    previous_nodes: Vec<String>,
}

fn part_one(input: &str) -> usize {
    let graph = parse(input);
    let mut queue = vec![];
    queue.push(State {
        previous_nodes: vec!["start".to_string()],
    });
    let mut ways = 0;
    while let Some(state) = queue.pop() {
        let current = state.previous_nodes.last().unwrap();
        if current == "end" {
            ways += 1;
            continue;
        }
        let neighbors = graph.get(current).unwrap();
        for neighbor in neighbors {
            // skip visited small caves
            if neighbor.chars().nth(0).unwrap().is_lowercase()
                && state.previous_nodes.contains(neighbor)
            {
                continue;
            }
            let mut new_visited = state.previous_nodes.clone();
            new_visited.push(neighbor.clone());
            queue.push(State {
                previous_nodes: new_visited,
            });
        }
    }

    ways
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
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/12_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 10);
        // assert_eq!(part_one(INPUT), 1665);

        // assert_eq!(part_two(INPUT_TEST), 195);
        // assert_eq!(part_two(INPUT), 235);
    }
}
