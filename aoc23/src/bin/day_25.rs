use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    ops::IndexMut,
};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/25_full.txt"));
const N_CUTS: usize = 3;

#[derive(Debug, Clone)]
struct Graph {
    node_edges: HashMap<String, Vec<String>>,
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
        let mut visited = vec![start.clone()];
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
        let group_a = self.get_all_connected(&self.node_edges.keys().next().unwrap());
        let group_b_start = self
            .node_edges
            .iter()
            .find(|node| !group_a.contains(&node.0))
            .unwrap();
        let group_b = self.get_all_connected(&group_b_start.0);

        group_a.len() * group_b.len()
    }
    pub fn get_neigbors(&self, key: &String) -> Vec<String> {
        self.node_edges.get(key).unwrap().clone()
    }
    pub fn unique_ways_between(&self, a: &String, b: &String) -> usize {
        let mut grid = self.clone();
        let mut ways = 0;
        while let Some(state) = grid.distance_between(a, b) {
            for pair in state.previous_nodes.windows(2) {
                grid.remove_edge(&pair[0], &pair[1]);
            }
            grid.remove_edge(state.previous_nodes.last().unwrap(), b);
            ways += 1;
            if ways > N_CUTS {
                return ways;
            }
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
        let a_node = self.node_edges.get_mut(a).unwrap();
        a_node.retain(|s| s != b);
        let b_node = self.node_edges.get_mut(b).unwrap();
        b_node.retain(|s| s != a);
    }
}

fn part_one(input: &str) -> usize {
    let mut graph = parse_components(input);
    let mut possible_connections = vec![];
    let names: Vec<String> = graph.node_edges.keys().map(|n| n.clone()).collect();
    for i in 0..names.len() {
        let name = &names[i];
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
    let pairs: Vec<(String, String)> = possible_connections
        .iter()
        .filter_map(|pair| {
            let ways = graph.unique_ways_between(&pair.0, &pair.1);
            if ways == N_CUTS {
                Some(pair.clone())
            } else {
                None
            }
        })
        .collect();
    // assert!(pairs.len() == N_CUTS);
    for pair in pairs {
        graph.remove_edge(&pair.0, &pair.1);
    }
    graph.count_two_groups()
}
fn parse_components(input: &str) -> Graph {
    let components: Vec<(String, Vec<String>)> = input
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
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for node in &components {
        if !m.contains_key(&node.0) {
            m.insert(node.0.clone(), node.1.clone());
        } else {
            let n = m.get_mut(&node.0).unwrap();
            for neighbor in &node.1 {
                n.push(neighbor.clone());
            }
        }
        // reverse
        for neighbor in &node.1 {
            if !m.contains_key(neighbor) {
                m.insert(neighbor.clone(), vec![node.0.clone()]);
            } else {
                let n = m.get_mut(neighbor).unwrap();
                n.push(node.0.clone());
            }
        }
    }
    Graph { node_edges: m }
}
fn main() {
    println!("1: {}", part_one(INPUT));
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/25_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 54);
        assert_eq!(part_one(INPUT), 527790);
    }
}
