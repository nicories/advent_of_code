use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    ops::IndexMut,
};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/25_full.txt"));

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<(String, Vec<String>)>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: String,
    previous_nodes: Vec<String>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub fn get_all_connected(&self, start: &String) -> Vec<String> {
        let mut queue: Vec<String> = vec![];
        let mut visited: Vec<String> = vec![start.clone()];
        for neighbor in self.get_neigbors(start) {
            queue.push(neighbor);
        }
        while let Some(node) = queue.pop() {
            for neighbor in self.get_neigbors(&node) {
                if !visited.contains(&neighbor) {
                    visited.push(neighbor.clone());
                    queue.push(neighbor);
                }
            }
        }
        visited
    }
    pub fn count_two_groups(&self) -> usize {
        let group_a = self.get_all_connected(&self.nodes[0].0);
        let group_b_start = self
            .nodes
            .iter()
            .find(|node| !group_a.contains(&node.0))
            .unwrap();
        let group_b = self.get_all_connected(&group_b_start.0);

        group_a.len() * group_b.len()
    }
    pub fn get_neigbors(&self, key: &String) -> Vec<String> {
        let mut v = vec![];
        for node in &self.nodes {
            if node.0 == key.to_string() {
                v.append(&mut node.1.clone());
            } else if node.1.contains(&key.to_string()) {
                v.push(node.0.clone());
            }
        }
        v
    }
    pub fn unique_ways_between(&self, a: &String, b: &String) -> usize {
        let mut grid = self.clone();
        let mut ways = 0;
        while let Some(state) = grid.distance_between(a, b) {
            for pair in state.previous_nodes[0..state.previous_nodes.len()].windows(2) {
                grid.remove_edge(&pair[0], &pair[1]);
            }
            grid.remove_edge(state.previous_nodes.last().unwrap(), b);
            ways += 1;
        }
        ways
    }
    pub fn distance_between(&self, a: &String, b: &String) -> Option<State> {
        let mut visited: Vec<String> = vec![];
        let mut queue: BinaryHeap<State> = BinaryHeap::new();
        for neighbor in self.get_neigbors(a) {
            queue.push(State {
                cost: 1,
                node: neighbor,
                previous_nodes: vec![a.clone()],
            });
        }
        while let Some(state) = queue.pop() {
            if &state.node == b {
                return Some(state);
            }
            for neighbor in self.get_neigbors(&state.node) {
                if !visited.contains(&neighbor) {
                    visited.push(neighbor.clone());
                    let mut v = state.previous_nodes.clone();
                    v.push(state.node.clone());
                    queue.push(State {
                        cost: state.cost + 1,
                        node: neighbor,
                        previous_nodes: v,
                    });
                }
            }
        }
        None
    }
    pub fn remove_edge(&mut self, a: &String, b: &String) {
        for node in &mut self.nodes {
            if node.0 == a.to_string() {
                if let Some(index) = node.1.iter().position(|key| key == b) {
                    node.1.remove(index);
                    return;
                }
            } else if node.0 == b.to_string() {
                if let Some(index) = node.1.iter().position(|key| key == a) {
                    node.1.remove(index);
                    return;
                }
            }
        }
        panic!("no edge found");
    }
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let mut graph = parse_components(input);
    let mut possible_connections = vec![];
    for i in 0..graph.nodes.len() {
        let name = graph.nodes[i].0.clone();
        for neighbor in graph.get_neigbors(&name) {
            let neighbors_level2 = graph.get_neigbors(&neighbor);
            if neighbors_level2
                .iter()
                .all(|n2| !graph.get_neigbors(&name.clone()).contains(n2))
            {
                if !possible_connections.contains(&(name.clone(), neighbor.clone()))
                    && !possible_connections.contains(&(neighbor.clone(), name.clone()))
                {
                    possible_connections.push((name.clone(), neighbor));
                }
            }
        }
    }
    let mut pairs: Vec<(String, String)> = vec![];
    for (i, pair) in possible_connections.iter().enumerate() {
        println!("{i} / {}", possible_connections.len());
        let ways = graph.unique_ways_between(&pair.0, &pair.1);
        if ways == 3 {
            pairs.push(pair.clone());
        }
        if pairs.len() == 3 {
            break;
        }
    }
    assert!(pairs.len() == 3);
    for pair in pairs {
        graph.remove_edge(&pair.0, &pair.1);
    }
    graph.count_two_groups()
}
fn parse_components(input: &str) -> Graph {
    let components = input
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(':').unwrap();
            let list: Vec<String> = rest
                .trim()
                .split(' ')
                .map(|x| x.trim())
                .map(|s| String::from(s))
                .collect();
            (name.to_string(), list)
        })
        .collect();
    Graph { nodes: components }
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/25_test.txt"));

    #[test]
    fn test() {
        {
            let mut graph = parse_components(INPUT_TEST);
            assert_eq!(graph.get_all_connected(&"hfx".to_string()).len(), 15);
            assert_eq!(
                graph.unique_ways_between(&"hfx".to_string(), &"pzl".to_string()),
                3
            );
            assert_eq!(
                graph.unique_ways_between(&"pzl".to_string(), &"hfx".to_string()),
                3
            );
            assert_eq!(
                graph.unique_ways_between(&"hfx".to_string(), &"ntq".to_string()),
                4
            );
            assert_eq!(
                graph
                    .distance_between(&"hfx".to_string(), &"pzl".to_string())
                    .unwrap()
                    .cost,
                1
            );
            assert_eq!(
                graph
                    .distance_between(&"pzl".to_string(), &"hfx".to_string())
                    .unwrap()
                    .cost,
                1
            );
            graph.remove_edge(&"pzl".to_string(), &"hfx".to_string());
            assert_eq!(
                graph
                    .distance_between(&"hfx".to_string(), &"pzl".to_string())
                    .unwrap()
                    .cost,
                4
            );
            assert_eq!(
                graph
                    .distance_between(&"pzl".to_string(), &"hfx".to_string())
                    .unwrap()
                    .cost,
                4
            );
            graph.remove_edge(&"bvb".to_string(), &"cmg".to_string());
            graph.remove_edge(&"nvd".to_string(), &"jqt".to_string());
            assert_eq!(
                graph.distance_between(&"hfx".to_string(), &"pzl".to_string()),
                None
            );
            assert_eq!(
                graph.distance_between(&"pzl".to_string(), &"hfx".to_string()),
                None
            );
            let n = graph.get_neigbors(&"bvb".to_string());
        }
        {
            let graph = parse_components(INPUT);
            let n = graph.get_neigbors(&"xtb".to_string());
        }
        assert_eq!(part_one(INPUT_TEST), 54);
        assert_eq!(part_one(INPUT), 527790);

        // assert_eq!(part_two(INPUT_TEST), 47.0);
        // assert_eq!(part_two(INPUT), 6542);
    }
}
