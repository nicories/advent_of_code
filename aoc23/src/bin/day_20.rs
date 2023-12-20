use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/20_full.txt"));
type Broadcaster = Vec<String>;

struct FlipFlop {
    name: String,
    state: bool,
    targets: Vec<String>,
}
impl FlipFlop {
    pub fn process_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        if pulse.state {
            vec![]
        } else {
            self.state = !self.state;
            self.targets
                .iter()
                .map(|target| Pulse {
                    state: self.state,
                    target: target.clone(),
                    source: self.name.clone(),
                })
                .collect()
        }
    }
}
#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    history: HashMap<String, bool>,
    targets: Vec<String>,
}
impl Conjunction {
    pub fn process_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        assert!(self.history.contains_key(&pulse.source));
        self.history.insert(pulse.source.clone(), pulse.state);
        let pulse_state = if self.history.values().all(|state| *state == true) {
            false
        } else {
            true
        };
        self.targets
            .iter()
            .map(|target| Pulse {
                state: pulse_state,
                target: target.clone(),
                source: self.name.clone(),
            })
            .collect()
    }
}

#[derive(Debug)]
struct Pulse {
    state: bool,
    source: String,
    target: String,
}

fn parse(
    input: &str,
) -> (
    Broadcaster,
    HashMap<String, FlipFlop>,
    HashMap<String, Conjunction>,
) {
    let mut broadcaster = None;
    let mut conjunctions = HashMap::new();
    let mut flipflops = HashMap::new();
    for line in input.lines() {
        let (name, rest) = line.split_once("->").unwrap();
        let targets: Vec<String> = rest
            .split(',')
            .map(|x| x.trim())
            .map(|s| s.to_string())
            .collect();
        let name = name.trim();
        if name == "broadcaster" {
            broadcaster = Some(targets);
        } else if name.starts_with('%') {
            flipflops.insert(
                name[1..name.len()].to_string(),
                FlipFlop {
                    name: name[1..name.len()].to_string(),
                    targets,
                    state: false,
                },
            );
        } else {
            conjunctions.insert(
                name[1..name.len()].to_string(),
                Conjunction {
                    name: name[1..name.len()].to_string(),
                    targets,
                    history: HashMap::new(),
                },
            );
        }
    }
    for i in 0..conjunctions.values().len() {
        let mut map = HashMap::new();
        let name = &conjunctions.values().nth(i).unwrap().name.clone();
        for (j, con) in conjunctions.iter().enumerate() {
            if con.1.targets.contains(name) {
                map.insert(con.1.name.clone(), false);
            }
        }
        conjunctions.values_mut().nth(i).unwrap().history = map;
    }
    for f in flipflops.values() {
        for target in &f.targets {
            if let Some(con) = conjunctions.get_mut(target) {
                con.history.insert(f.name.clone(), false);
            }
        }
    }
    (broadcaster.unwrap(), flipflops, conjunctions)
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let (broadcaster, mut flipflops, mut conjunctions) = parse(input);
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
    let mut low_sum = 0;
    let mut high_sum = 0;
    for i in 0..1000 {
        // button to broadcaster
        low_sum += 1;

        for target in &broadcaster {
            pulse_queue.push_back(Pulse {
                source: "broadcaster".to_string(),
                state: false,
                target: target.clone(),
            });
        }
        while let Some(pulse) = pulse_queue.pop_front() {
            if pulse.state {
                high_sum += 1;
            } else {
                low_sum += 1;
            }
            if let Some(flipflop) = flipflops.get_mut(&pulse.target) {
                for pulse in flipflop.process_pulse(&pulse) {
                    pulse_queue.push_back(pulse);
                }
            }
            if let Some(conjunction) = conjunctions.get_mut(&pulse.target) {
                for pulse in conjunction.process_pulse(&pulse) {
                    pulse_queue.push_back(pulse);
                }
            }
        }
    }

    low_sum * high_sum
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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/20_test.txt"));
    const INPUT_TEST2: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/20_test2.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 32000000);
        assert_eq!(part_one(INPUT_TEST2), 11687500);
        assert_eq!(part_one(INPUT), 711650489);

        // assert_eq!(part_two(INPUT_TEST), 167409079868000);
        // assert_eq!(part_two(INPUT), 138625360533574);
    }
}
